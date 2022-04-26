# Java implementation
> Requires JDK and Java
## Build and run:
```bash
make build
```

or

```bash
# create directory if it doesnt already exist
mkdir -p dist

# compile classes
javac src/*.java

# move compiled files to dist
mv src/*.class dist/

# run Xc on all files in /java
cd dist/ && java Xc ../*
```
