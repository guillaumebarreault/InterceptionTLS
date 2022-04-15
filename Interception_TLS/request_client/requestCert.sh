custom_$1

# create private_key of custom:
openssl genrsa -out customCA_$1.key

# generate crs:
openssl req -new -key customCA_$1.key -out customCA_$1.csr

# CA side: 
cp customCA_$1.csr ../CA
rm -fv customCA_$1.csr
chmod 400 customCA_$1.key
cd ../CA

# check request:
openssl req -text -noout -verify -in customCA_$1.csr | grep Subject:

# sign and generate cert:
openssl ca -config ca-ssl.conf -out customCA_$1.crt -in customCA_$1.csr

# remove request:
rm -f customCA_$1.csr

