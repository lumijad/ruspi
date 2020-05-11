systemctl stop ruspi.service
sleep 2

cp ./target/release/webserver /opt/ruspi/webserver -v
cp -r ./config /opt/ruspi/ -v
cp -r ./static /opt/ruspi/ -v
cp -r ./templates /opt/ruspi/ -v
cp -r ./tls /opt/ruspi/ -v
cp prod.env /opt/ruspi/.env -v
chown -R ruspi /opt/ruspi

systemctl start ruspi.service
systemctl status ruspi.service
