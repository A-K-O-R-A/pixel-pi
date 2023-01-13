cd pi_data

echo "Downloading dec 1m"
curl https://www.pi2e.ch/blog/wp-content/uploads/2017/03/pi_dec_1m.txt -o pi_dec_1m.txt

echo "Downloading hex 1m"
curl https://www.pi2e.ch/blog/wp-content/uploads/2017/03/pi_hex_1m.txt -o pi_hex_1m.txt

echo "For 1b dec pi files download the compressed version from https://stuff.mit.edu/afs/sipb/contrib/pi/pi-billion.txt"
echo "For 1b hex pi files download the compressed version from https://archive.org/download/pi_hex_1b/pi_hex_1b.zip"