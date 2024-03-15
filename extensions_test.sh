for extension in "png" "jpg" "gif" "bmp" "tif" "ppm"
do
    body="18
    d
    - -
    100 70 70
    100 100 70
    200 200 160
    n
    test"
    echo -e "${body}\n${extension}" | ./target/debug/dragon
done
