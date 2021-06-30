mkdir bin
for f in *.go
do
		go build -ldflags="-s -w" $f
		mv ${f//.go/} bin
done
