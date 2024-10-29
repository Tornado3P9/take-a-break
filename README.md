# take-a-break


## Pure Rust:
```bash
# Play a sound and display a popup message after 45 minutes:
take-a-break 45
```

## Different options include:

- using a bash script

```sh
#!/bin/bash

# To remove an `at` job after it has been started, you need to first find the job's identification number using the `atq` command,
# which lists the current user's pending jobs. Once you have the job number, you can delete it using the atrm command.
# Here's how you can do it:
#  1. List the scheduled jobs to find the job ID (Show jobs of all users by using `sudo`):
#  atq
#  2. Remove the job using its ID (replace jobid with the actual job number):
#  atrm jobid

# Full path to your sound file
SOUND_FILE="${HOME}/Music/sounds/new-message.ogg"

# Full paths to commands
NOTIFY_SEND="/usr/bin/notify-send"
PLAYER="/usr/bin/paplay"

# Function to play sound
play_sound() {
    if [ -r "$SOUND_FILE" ]; then
        $NOTIFY_SEND -t 10000 "Reminder:" -i dialog-information "Trink was, beweg dich, geh raus"
        $PLAYER "$SOUND_FILE"
    else
        echo "Sound file does not exist or is not readable."
    fi
}

# Test the play_sound function
play_sound

# Schedule the sound to play one hour after login
at now + 1 hour <<EOF
$(typeset -f play_sound)
play_sound
EOF

# Set script permissions to owner only
# This line should be run outside of the script to set its permissions
# chmod 700 /path/to/this/script.sh
```

- using a [cron job](https://github.com/Tornado3P9/Linux-Console-Tools/blob/master/admin%20tools/Cron.md)

- using programs like [Safe Eyes](https://slgobinath.github.io/SafeEyes/)
