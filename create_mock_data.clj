#!/usr/bin/env bb

;; Seeds the back-end with mock events via `POST /events`.
;;
;; Usage:
;;   SS_API_TOKEN=... bb create_mock_data.clj [api-url]
;;
;; The API URL is taken from the first argument, else SS_API_URL, else the
;; `api` output in .sst/outputs.json (written by `sst dev` / `sst deploy`).
;; Events have fixed IDs, so re-running overwrites them rather than adding
;; duplicates. Dates are relative to today, so the events stay upcoming.

(require '[babashka.fs :as fs]
				 '[babashka.http-client :as http]
				 '[cheshire.core :as json]
				 '[clojure.string :as str])

(import '[java.time LocalDate LocalTime ZoneOffset])

(def outputs-file (str (fs/path (fs/parent *file*) ".sst" "outputs.json")))

(defn die! [& msg]
	(binding [*out* *err*]
		(println (str/join " " msg)))
	(System/exit 1))

(defn env [k]
	(some-> (System/getenv k) str/trim not-empty))

(defn api-url [args]
	(or (first args)
			(env "SS_API_URL")
			(when (fs/exists? outputs-file)
				(:api (json/parse-string (slurp outputs-file) true)))
			(die! "No API URL: pass one as an argument, set SS_API_URL, or run `sst dev` first.")))

;; Epoch millis for `days` from today at `time` (HH:mm, UTC).
(defn at [days time]
	(-> (LocalDate/now ZoneOffset/UTC)
			(.plusDays days)
			(.atTime (LocalTime/parse time))
			(.toInstant ZoneOffset/UTC)
			(.toEpochMilli)))

;; picsum.photos returns a stable random photo per seed.
(defn image [id alt]
	{:id id :alt alt :url (str "https://picsum.photos/seed/" id "/1200/675")})

(def mock-events
	[;; Portugal
	 {:id "hyqsrkczb1q2zkhlidd7gwqm"
		:eventType "TRAIL_RUN"
		:name "Sintra Trail Series: Round 4"
		:description "A 12 km and 21 km mountain trail run with water points every 4 km. Medals for all finishers."
		:startDate (at 5 "06:00") :endDate (at 5 "13:00")
		:distanceMin 12 :distanceMax 21
		:address {:countryCode "PT" :addressLine1 "Parque de Monserrate" :cityTownLocality "Sintra"
							:administrativeArea "Lisboa" :postalCode "2710-405" :latitude 38.7939 :longitude -9.4209}
		:coverImage (image "pt-trail-cover" "Runners on a mountain path at sunrise")
		:images [(image "pt-trail-1" "Start line") (image "pt-trail-2" "Runner crossing a stream")]}
	 {:id "fmbrp6aj9vwdojs6p7r4ywpt"
		:eventType "MOUNTAIN_BIKE"
		:name "Arrábida Enduro Challenge"
		:description "Five timed enduro stages along the Arrábida ridge, with shuttle uplift between stages."
		:startDate (at 12 "07:30") :endDate (at 12 "16:00")
		:distanceMin 35 :distanceMax 35
		:address {:countryCode "PT" :addressLine1 "Estrada de Escarpa" :cityTownLocality "Setúbal"
							:sublocality "Azeitão" :administrativeArea "Setúbal" :postalCode "2925-001"}
		:coverImage (image "pt-mtb-cover" "Rider descending a rocky trail")}
	 {:id "a0m4h84k3ghbp0dl5r5kvmfl"
		:eventType "HIKE"
		:name "Rota Vicentina Coastal Walk"
		:description "A guided 18 km walk along the Fishermen's Trail cliffs, finishing with lunch in Zambujeira do Mar."
		:startDate (at 19 "08:00") :endDate (at 19 "15:00")
		:distanceMin 18 :distanceMax 18
		:address {:countryCode "PT" :addressLine1 "Largo da Igreja" :cityTownLocality "Odemira"
							:administrativeArea "Beja" :postalCode "7630-139"}
		:coverImage (image "pt-hike-cover" "Cliff path above the Atlantic")}
	 {:id "p7nn29bf5t9e4q4ppjt09k5b"
		:eventType "EXHIBITION"
		:name "Porto Outdoor Film Festival"
		:description "Two days of adventure film screenings, gear stands and talks from local athletes."
		:startDate (at 26 "10:00") :endDate (at 27 "22:00")
		:address {:countryCode "PT" :addressLine1 "Rua de Dom Manuel II" :cityTownLocality "Porto"
							:administrativeArea "Porto" :postalCode "4050-346"}
		:coverImage (image "pt-expo-cover" "Audience watching a film screening")}

	 ;; Germany
	 {:id "f3w0mjbrhkumoxxks3nc9uem"
		:eventType "EXHIBITION"
		:name "Outdoor Gear Expo"
		:description "Over 80 exhibitors showing the latest running, cycling and hiking gear, plus talks from pro athletes."
		:startDate (at 8 "09:00") :endDate (at 9 "18:00")
		:address {:countryCode "DE" :addressLine1 "Messedamm 22" :cityTownLocality "Berlin" :sublocality "Westend"
							:administrativeArea "Berlin" :postalCode "14055" :latitude 52.5065 :longitude 13.2846}
		:coverImage (image "de-expo-cover" "Exhibition hall full of stands")
		:images [(image "de-expo-1" "Gear stand") (image "de-expo-2" "Talk on the main stage")]}
	 {:id "vribivf08d4gg1xsp9gfk1nc"
		:eventType "TRAIL_RUN"
		:name "Harz Forest Trail Marathon"
		:description "Marathon and half-marathon routes through the Harz forest, climbing to the Brocken summit."
		:startDate (at 15 "07:00") :endDate (at 15 "16:00")
		:distanceMin 21 :distanceMax 42
		:address {:countryCode "DE" :addressLine1 "Brockenstraße 1" :cityTownLocality "Wernigerode"
							:administrativeArea "Sachsen-Anhalt" :postalCode "38855"}
		:coverImage (image "de-trail-cover" "Runners in a pine forest")}
	 {:id "tw25p1stuxvbcg8ngdb5qhzv"
		:eventType "MOUNTAIN_BIKE"
		:name "Black Forest MTB Marathon"
		:description "Classic cross-country marathon with 60 km and 90 km loops through the Black Forest."
		:startDate (at 22 "08:00") :endDate (at 22 "17:00")
		:distanceMin 60 :distanceMax 90
		:address {:countryCode "DE" :addressLine1 "Schulstraße 5" :cityTownLocality "Kirchzarten"
							:administrativeArea "Baden-Württemberg" :postalCode "79199"}
		:coverImage (image "de-mtb-cover" "Riders climbing a forest road")}
	 {:id "xoi7sr417p95dsivtdatu9iq"
		:eventType "HIKE"
		:name "Saxon Switzerland Sunrise Hike"
		:description "An early start to catch sunrise from the Bastei bridge, then a loop through the sandstone towers."
		:startDate (at 33 "05:30") :endDate (at 33 "12:00")
		:distanceMin 14 :distanceMax 14
		:address {:countryCode "DE" :addressLine1 "Bastei 1" :cityTownLocality "Lohmen"
							:administrativeArea "Sachsen" :postalCode "01847"}
		:coverImage (image "de-hike-cover" "Sandstone towers at sunrise")}

	 ;; Switzerland
	 {:id "zbktmj21skghyumsaf39vvmt"
		:eventType "HIKE"
		:name "Lauterbrunnen Valley Waterfall Hike"
		:description "A guided 16 km hike past the valley's waterfalls, climbing to Mürren for views of the Eiger, Mönch and Jungfrau."
		:startDate (at 6 "08:00") :endDate (at 6 "15:00")
		:distanceMin 16 :distanceMax 16
		:address {:countryCode "CH" :addressLine1 "Bahnhofplatz" :cityTownLocality "Lauterbrunnen"
							:administrativeArea "Bern" :postalCode "3822" :latitude 46.5935 :longitude 7.9091}
		:coverImage (image "ch-hike-cover" "Waterfall dropping into a glacial valley")}
	 {:id "kp8nasxohg4s32wabiw7qfde"
		:eventType "MOUNTAIN_BIKE"
		:name "Verbier Alpine Enduro"
		:description "Lift-assisted enduro with four timed descents from the Les Ruinettes plateau, over 2,500 m of downhill."
		:startDate (at 13 "08:30") :endDate (at 13 "16:30")
		:distanceMin 40 :distanceMax 40
		:address {:countryCode "CH" :addressLine1 "Place Centrale" :cityTownLocality "Verbier"
							:administrativeArea "Valais" :postalCode "1936"}
		:coverImage (image "ch-mtb-cover" "Rider on an alpine singletrack")}
	 {:id "kdtu72svznn62kr87w0egejr"
		:eventType "TRAIL_RUN"
		:name "Zermatt Matterhorn Trail"
		:description "High-alpine 20 km and 45 km courses with Matterhorn views, topping out above 2,800 m."
		:startDate (at 20 "06:30") :endDate (at 20 "17:00")
		:distanceMin 20 :distanceMax 45
		:address {:countryCode "CH" :addressLine1 "Bahnhofplatz 5" :cityTownLocality "Zermatt"
							:administrativeArea "Valais" :postalCode "3920" :latitude 46.0207 :longitude 7.7491}
		:coverImage (image "ch-trail-cover" "Runners below the Matterhorn")}
	 {:id "jikye1bfyw8muw4aaem72pc5"
		:eventType "EXHIBITION"
		:name "Zurich Mountain Sports Fair"
		:description "Climbing, ski touring and trail gear from over 60 brands, plus talks from alpine guides."
		:startDate (at 30 "10:00") :endDate (at 31 "18:00")
		:address {:countryCode "CH" :addressLine1 "Wallisellenstrasse 49" :cityTownLocality "Zurich"
							:sublocality "Oerlikon" :administrativeArea "Zurich" :postalCode "8050"}
		:coverImage (image "ch-expo-cover" "Climbing wall at an exhibition stand")}])

(defn wait-for-health! [url]
	(loop [attempt 1]
		(let [ok? (try (= 200 (:status (http/get (str url "/health") {:throw false :timeout 5000})))
									 (catch Exception _ false))]
			(cond
				ok? :ok
				(>= attempt 30) (die! "API at" url "did not become healthy.")
				:else (do (println "Waiting for API health check…")
									(Thread/sleep 2000)
									(recur (inc attempt)))))))

(defn save-event! [url token event]
	(let [{:keys [status body]} (http/post (str url "/events")
																				 {:throw false
																					:headers {"content-type" "application/json"
																										"x-api-token" token}
																					:body (json/generate-string {:event event})})]
		(println (format "  %s %-4s %-14s %s"
										 (if (= 200 status) "✓" "✗")
										 (get-in event [:address :countryCode])
										 (:eventType event)
										 (:name event)))
		(when-not (= 200 status)
			(println "    HTTP" status body))
		(= 200 status)))

(let [token (or (env "SS_API_TOKEN") (die! "SS_API_TOKEN is not set or empty."))
			url (str/replace (api-url *command-line-args*) #"/+$" "")]
	(println "Seeding" (count mock-events) "mock events to" url)
	(wait-for-health! url)
	(let [failed (->> mock-events
										(map #(save-event! url token %))
										(remove true?)
										count)]
		(println (- (count mock-events) failed) "saved," failed "failed.")
		(when (pos? failed) (System/exit 1))))
