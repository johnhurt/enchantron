#!/bin/bash

while getopts ":s:t:p:l:D:" opt; do

    case ${opt} in
        s )
            SOURCE=$OPTARG
            ;;
        t )
            TARGET=$OPTARG
            ;;
        p )
            PLATFORM=$OPTARG
            ;;
        l )
            LIBRARY=$OPTARG
            ;;
        D )
            PREPROCESSOR_DEFINE="-D $OPTARG"
            ;;
        \? )
            echo "Invalid option: $OPTARG" 1>&2
            exit 1
            ;;
        : )
            echo "Invalid option: $OPTARG requires an argument" 1>&2
            exit 1
            ;;
    esac

done

shift $((OPTIND -1))

if [ -z ${LIBRARY+x} ]
then
    echo "library name (-l) is required" 1>&2
    exit 1
fi

if [ -z ${PLATFORM+x} ]
then
    echo "platform name (-p) is required" 1>&2
    exit 1
fi

if [ -z ${TARGET+x} ]
then
    echo "target director name (-t) is required" 1>&2
    exit 1
fi

if [ -z ${SOURCE+x} ]
then
    echo "source director name (-s) is required" 1>&2
    exit 1
fi

FILES_BASES=$@

TARGET_DIR="$TARGET/$PLATFORM"
METAL_BUILD_DIR="$TARGET_DIR/build"
AIR_FILES=

mkdir -p $METAL_BUILD_DIR

set -e

for f in $FILES_BASES
do
    METAL_FILE="$SOURCE/$f.metal"
    AIR_FILE="$METAL_BUILD_DIR/$f.air"
    AIR_FILES="$AIR_FILES $AIR_FILE"
    
    xcrun -sdk $PLATFORM metal -ffast-math -c "$METAL_FILE" -o "${AIR_FILE}" $PREPROCESSOR_DEFINE
    
done

xcrun -sdk $PLATFORM metallib $AIR_FILES -o "$TARGET_DIR/${LIBRARY}.metallib"
