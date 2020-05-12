addgroup ruspi
sudo useradd -M ruspi -g ruspi

mkdir /opt/ruspi
cp ./target/release/webserver /opt/ruspi/webserver
cp -r ./config /opt/ruspi/
cp -r ./static /opt/ruspi/
cp -r ./templates /opt/ruspi/
cp -r ./tls /opt/ruspi/ls
cp prod.env /opt/ruspi/.env
chown -R ruspi /opt/ruspi

cp ./scripts/ruspi.service /etc/systemd/system/ -v
systemctl enable ruspi.service
systemctl start ruspi.service
