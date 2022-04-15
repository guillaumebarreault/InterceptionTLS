# private key of CA => must be protected; can sign new cert.
# create Kpr_CA:
openssl genrsa -out CA-ROOT.key

# request signature from Kpr_CA:
openssl req -new -key CA-ROOT.key -out CA-ROOT.csr -sha256

# generate cert from Kpr_CA: (auto_sign)
openssl x509 -req -days 365 -in CA-ROOT.csr -out CA-ROOT.crt -signkey CA-ROOT.key

# we have 3 files:
# CA-ROOT.key => must be protected
chmod 400 CA-ROOT.key
# CA-ROOT.crt
# CA-ROOT.csr => we can remov
rm -fv CA-ROOT.csr

# create config file of CA:
vim ca-ssl.conf

# add 2 files:
mkdir indexs
cd indexs
touch index.txt     # incremente cert number
cd ..
mkdir serials
cd serials
echo '01' > serial  # keep trace of each cert edit
cd ..


