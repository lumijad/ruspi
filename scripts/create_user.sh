sudo useradd -M ruspi

sudo systemctl daemon-reload

sudo systemctl enable ruspi.service
sudo systemctl status ruspi.service
sudo systemctl start ruspi.service
sudo systemctl stop ruspi.service

journalctl -xe