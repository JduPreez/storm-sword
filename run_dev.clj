#!/usr/bin/env bb

;; Usage: ./run_dev.clj [mock-data]
;;
;;   mock-data  Once `sst dev` has finished deploying, seed the API with
;;              create_mock_data.clj. Its output goes to .sst/log/mock-data.log
;;              so it doesn't break the sst dev terminal UI.

(require '[babashka.fs :as fs]
				 '[babashka.process :as p]
				 '[clojure.string :as str])

(def root (fs/parent *file*))
(def outputs-file (fs/path root ".sst" "outputs.json"))
(def mock-data-log (fs/path root ".sst" "log" "mock-data.log"))

(defn ssrun! [cmd]
	(let [proc (p/process cmd {:inherit true})
				result @proc]
		(when-not (zero? (:exit result))
			(throw (ex-info (str "Command failed: " (str/join " " cmd))
											{:cmd cmd
											 :exit (:exit result)})))))

(defn mtime [path]
	(when (fs/exists? path)
		(.toMillis (fs/last-modified-time path))))

;; `sst dev` rewrites .sst/outputs.json once its deploy finishes, so wait for
;; that rather than seeding against a stale deployment (e.g. an old token).
(defn wait-for-deploy! [since timeout-ms]
	(let [deadline (+ (System/currentTimeMillis) timeout-ms)]
		(loop []
			(let [m (mtime outputs-file)]
				(cond
					(and m (> m (or since 0))) :deployed
					(> (System/currentTimeMillis) deadline) :timed-out
					:else (do (Thread/sleep 2000) (recur)))))))

(defn seed-mock-data-after-deploy! [since]
	(future
		(fs/create-dirs (fs/parent mock-data-log))
		(let [status (wait-for-deploy! since (* 10 60 1000))]
			(spit (str mock-data-log)
						(str "Deploy wait: " (name status) "\n"))
			@(p/process ["bb" (str (fs/path root "create_mock_data.clj"))]
									{:dir (str root)
									 :out :append :out-file (fs/file mock-data-log)
									 :err :append :err-file (fs/file mock-data-log)}))))

(let [token (some-> (System/getenv "SS_API_TOKEN") str/trim not-empty)
			mock-data? (some #{"mock-data"} *command-line-args*)
			unknown (remove #{"mock-data"} *command-line-args*)]
	(when-not token
		(binding [*out* *err*]
			(println "SS_API_TOKEN is not set or empty."))
		(System/exit 1))
	(when (seq unknown)
		(binding [*out* *err*]
			(println "Unknown argument(s):" (str/join " " unknown))
			(println "Usage: ./run_dev.clj [mock-data]"))
		(System/exit 1))

	(ssrun! ["sst" "secret" "set" "EventsApiToken" token])
	(when mock-data?
		(println "Mock data will be seeded once sst dev has deployed; see" (str mock-data-log))
		(seed-mock-data-after-deploy! (mtime outputs-file)))
	(ssrun! ["sst" "dev"])
	(shutdown-agents))
