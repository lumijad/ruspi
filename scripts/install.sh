mkdir /opt/ruspi
cp ./target/release/webserver /opt/ruspi/webserver
cp -r ./config /opt/ruspi/config
cp -r ./static /opt/ruspi/static
cp -r ./templates /opt/ruspi/templates
cp -r ./tls /opt/ruspi/tls
cp prod.env /opt/ruspi/.env

