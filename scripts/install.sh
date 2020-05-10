addgroup ruspi
sudo useradd -M ruspi -g ruspi

mkdir /opt/ruspi
cp ./target/release/webserver /opt/ruspi/webserver
cp -r ./config /opt/ruspi/config
cp -r ./static /opt/ruspi/static
cp -r ./templates /opt/ruspi/templates
cp -r ./tls /opt/ruspi/tls
cp prod.env /opt/ruspi/.env
chown -R ruspi /opt/ruspi

cp ./scripts/ruspi.service /etc/systemd/system/ -v
systemctl enable ruspi.service
systemctl start ruspi.service
