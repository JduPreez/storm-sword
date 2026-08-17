#!/usr/bin/env bb

(require '[babashka.process :as p]
				 '[clojure.string :as str])

(defn ssrun! [cmd]
	(let [proc (p/process cmd {:inherit true})
				result @proc]
		(when-not (zero? (:exit result))
			(throw (ex-info (str "Command failed: " (str/join " " cmd))
											{:cmd cmd
											 :exit (:exit result)})))))

(let [token (some-> (System/getenv "SS_API_TOKEN") str/trim not-empty)]
	(when-not token
		(binding [*out* *err*]
			(println "SS_API_TOKEN is not set or empty."))
		(System/exit 1))

	(ssrun! ["sst" "secret" "set" "EventsApiToken" token])
	(ssrun! ["sst" "dev"]))
