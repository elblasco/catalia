;; 4 integers
;; v the node value
;; w type of node, either leaf or node
;; x is left sub-tree sorted?
;; y is right sub-tree sorted?

(
 ( "treeOfInt"
   ( "nodetreeOfInt"
	 ( (v lv lw lx ly rv rw rx ry)
	   v
	   1
	   (ite
		(or
		 (= lw 0)
		 (and (= lw 1) (<= lv v) (= lx 1) (= ly 1)))
		1
		0)
	   (ite
		(or
		 (= rw 0)
		 (and (= rw 1) (> rv v) (= rx 1) (= ry 1)))
		1
		0))
	 )
   ( "leaftreeOfInt"
	 ( ()
	   0
	   0
	   1
	   1
	   )
	 )
   )
 )
