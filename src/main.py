import sys, re

for line in sys.stdin:
	cells = line.split(",")

	vect = []
	for c in cells:
		c = c.replace("\n","")
		m = re.search('[0-9]{3}-[0-9]{2}-[0-9]{4}', c)
		if m is not None:
			c = "XXX-XX-XXXX"
		vect += [c]

	print(','.join(vect))