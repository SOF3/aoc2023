SHELL = /usr/bin/env bash
test:
	yq eval -ojson .part$(PART).input ../data/day$(DAY).yaml | \
		jq -f d$(DAY)p$(PART).jq -L. \
		>output/d$(DAY)p$(PART).txt
	diff <(yq eval .part$(PART).output ../data/day$(DAY).yaml) output/d$(DAY)p$(PART).txt
run:
	jq -f d$(DAY)p$(PART).jq -sR ../data/day$(DAY).txt -L.
