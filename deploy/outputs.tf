output "server_ipv4" {
  value = hcloud_server.freebsd.ipv4_address
}

output "server_ipv6" {
  value = hcloud_server.freebsd.ipv6_address
}

output "ssh_user" {
  value = "root"
}