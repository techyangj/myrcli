# this is my rust cli program
## some realizations here 
1. convert a csv file to a json or yaml file
2. generate a random password
3. generate a web page into a a image with the qrcode(Algorithm implementation in progress)
## Some instructions 
### for conver a csv file
1. cargo run -- csv -i assets/juventus.csv
2. cargo run -- csv -i assets/juventus.csv  --format yaml
### for random password
1. cargo run -- genpass -length 16
2. cargo run -- genpass --length 2
### for web to image
1. cargo run -- web2images --url https://github.com/dtolnay/anyhow --output ./output.png