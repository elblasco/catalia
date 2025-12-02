;; cons(a, nil) |-> forall x,y,z:int. (1, (ite (= y 0) x (ite (>= x z) x z)))
;; a |-> (w, x)     l |-> (y,z)

;; Every tree is reduced to 3 integers
;; (v, w, x) 
;; v node value
;; w is the constuctor cons?
;; x Is it sorted so far?
(
 ( "treeOfInt"
   ( "nodetreeOfInt"
	 ( (v lv lw lx rv rw rx)
	   v
	   1
	   (ite ;; The node is orted if
		(or
		 ;; The sub-trees are leaves
		 (and (= lw 0) (= rw 0))
		 ;; It has only the left sub-tree, the sub-tree is sorted and the values are sorted
		 (and (= lw 1) (= rw 0) (= lx 1) (> v lv))
		 ;; It has only the right sub-tree, the sub-tree is sorted and the values are sorted
		 (and (= lw 0) (= rw 1) (= rx 1) (< v rv))
		 ;; It has two sub-trees, the sub-trees are sorted and the values are sorted
		 (and (= lw 1) (= rw 1) (= rx 1) (= lx 1) (> v lv) (< v rv)))
		1
		0
		)
	   )
	 )
   ( "leaftreeOfInt"
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
