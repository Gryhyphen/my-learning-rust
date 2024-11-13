# Notes

This is a triumph

# Good scripts

## The Best Script

This script builds the application, and if successful, deploys it to the rp2040.

Note - I had to make it executable using `chmod +x /workspaces/myFirstBlinky/MyFirstRustBlinky/embassy_blinky/quickdirtybuild.sh`

Note - The -E is required to keep the current environment context when the script is running
	   otherwise the script can't find things like the toolchain and rust.

```shell
sudo -E ./quickdirtybuild.sh
```