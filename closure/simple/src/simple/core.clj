(ns simple.core
  (:require [quil.core :as q]))

(defn setup []
  (q/frame-rate 1)
  (q/background 200))

(defn draw []
  (q/camera 150 150 150 0 0 0 0 0 1)
  (q/box 100)
  (q/save "generated/test.jpeg")
  (q/exit))

(q/defsketch example
  :title "testing"
  :setup setup
  :draw draw
  :size [600 800]
  :renderer :p3d)

