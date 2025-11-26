;; cons(a,l) a|->(w) and l |->(x, y, z)

;; w curr elem
;; x is it cons?
;; y previous elem
;; z are previous elements sorted?
(
 ( "listOfInt"
   ( "conslistOfInt"
	 ( (w x y z)
	   1
	   w
	   (ite (or (= x 0) (and (<= w y) (= z 1))) 1 0)
	   )
	 )
   ( "nillistOfInt"
	 ( ()
	   0
	   0
	   1
	   )
	 )
   )

 ( "foobar"
   ( "foo"
	 ( (a b)
	   1
	   )
	 )
   ( "bar"
	 ( ()
	   0
	   )
	 )
   )
 )
