process()
{
export base=`basename $1`
export base="${base%.*}"
python3 convert.py $1 ${base}_compressed.h  ${base}_decl.h  $base
}

process ../rust_logo.gif
