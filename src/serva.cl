(ql:quickload '(hunchentoot cl-json))
(hunchentoot:start (make-instance 'hunchentoot:easy-acceptor))
(hunchentoot:define-easy-handler (say-hello :uri "/hello") (cname) 
     (setf (hunchentoot:content-type*) "text/plain")
     (format nil "Hello dear ~a, I am Rakshit. This website was built using Common Lisp." cname))
(defclass listings()
  ((product :accessor product
            :initarg :product)
   (store :accessor store
          :initarg :store)
   (price :accessor price
          :initarg :price)))
(defvar phone 
  (make-instance 'listings
                 :product "LG G10"
                 :store "Amazon"
                 :price "Rs.30,000"))

;; Static Replier
(hunchentoot:define-easy-handler (say-phone :uri "/phone") () 
        (setf (hunchentoot:content-type*) "application/json")
        (json:encode-json-to-string phone))

;; Dynamic generator
(hunchentoot:define-easy-handler (say-dynamic :uri "/find") (q)
        (setf (hunchentoot:content-type*) "application/json")
        (json:encode-json-to-string 
          (make-instance 'listings
                         :product q
                         :store "Flippykart"
                         :price 30000)))
