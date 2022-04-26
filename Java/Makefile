all: pre
	# compile classes
	javac src/*.java
	# move compiled files to dist
	mv src/*.class dist/
	# run Xc on all files in /java
	cd dist/ && java Xc ../*

pre:
	mkdir -p dist

clean:
	rm -r dist
