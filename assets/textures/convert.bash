#! /bin/bash

mv *.bmp /tmp
mv *.png /tmp

set -e
size=48

for f in *.svg; do
	inkscape -z -w $size -h $size $f -e $(echo $f | sed 's/.svg/.png/g');
	convert $(echo $f | sed 's/.svg/.png/g') $(echo $f | sed 's/.svg/.bmp/g');
done

mv *.png /tmp
