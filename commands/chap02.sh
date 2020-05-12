#!/bin/sh

INPUT_FILE_NAME="../data/popular-names.txt"
OUTPUT_DIR=../data/chap02_expected
N=5

# Preparing...
echo "Cleaning previous output files..."
rm -r $OUTPUT_DIR

echo "Creating output directory..."
mkdir $OUTPUT_DIR

# Executing...
echo "-----------------------------"
echo "Chapter 2-10 "

cat $INPUT_FILE_NAME | wc -l | sed -e 's/ //g' > $OUTPUT_DIR/10.txt

cat $OUTPUT_DIR/10.txt
echo "-----------------------------"
echo "Chapter 2-11"

## sed command for macOS. If using Linux, use "\t" for tab character
cat $INPUT_FILE_NAME | sed -e 's/	/ /g' > $OUTPUT_DIR/11_sed.txt
## tr command
cat $INPUT_FILE_NAME | tr "\t" " " > $OUTPUT_DIR/11_tr.txt
## expand command
expand -t 1 $INPUT_FILE_NAME > $OUTPUT_DIR/11_expand.txt

ls -l $OUTPUT_DIR/11_*
echo "-----------------------------"
echo "Chapter 2-12"
cat $INPUT_FILE_NAME | cut -f 1 > $OUTPUT_DIR/12_col1.txt
cat $INPUT_FILE_NAME | cut -f 2 > $OUTPUT_DIR/12_col2.txt

ls -l $OUTPUT_DIR/12_*
echo "-----------------------------"
echo "Chapter 2-13"

paste $OUTPUT_DIR/12_col1.txt $OUTPUT_DIR/12_col2.txt > $OUTPUT_DIR/13.txt

ls -l $OUTPUT_DIR/13*
echo "-----------------------------"
echo "Chapter 2-14"

head -n $N $INPUT_FILE_NAME > $OUTPUT_DIR/14.txt

ls -l $OUTPUT_DIR/14.txt
echo "-----------------------------"
echo "Chapter 2-15"

tail -n $N $INPUT_FILE_NAME > $OUTPUT_DIR/15.txt

ls -l $OUTPUT_DIR/15.txt
echo "-----------------------------"
echo "Chapter 2-16"

LINES=`cat $OUTPUT_DIR/10.txt`
SPLIT_LINES=`echo $LINES/$N | bc`
split -a 1 -l $SPLIT_LINES $INPUT_FILE_NAME $OUTPUT_DIR/16_

ls -l $OUTPUT_DIR/16_*
echo "-----------------------------"
echo "Chapter 2-17"

cat $INPUT_FILE_NAME | cut -f 1 | sort | uniq | wc -l | sed -e 's/ //g' > $OUTPUT_DIR/17.txt

cat $OUTPUT_DIR/17.txt
echo "-----------------------------"
echo "Chapter 2-18"

cat $INPUT_FILE_NAME | sort -g -r -k 3 > $OUTPUT_DIR/18.txt

ls -l $OUTPUT_DIR/18.txt
echo "-----------------------------"
echo "Chapter 2-19"

cat $INPUT_FILE_NAME | cut -f 1 | sort | uniq -c | sort -r > $OUTPUT_DIR/19.txt

ls -l $OUTPUT_DIR/19.txt
echo "Finished"