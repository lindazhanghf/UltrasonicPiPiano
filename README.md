# Ultrasonic Light Piano

This project is based on Ultrasonic Pi Piano by Andy Grove. Please refer to the detailed [instructable](https://www.instructables.com/id/Ultrasonic-Pi-Piano-With-Gesture-Controls/) with full information on how to make this project.

# Video

[![Raspberry Light Piano](https://img.youtube.com/vi/eIBYThDnX6c/0.jpg)](https://youtu.be/eIBYThDnX6c)

# Build the program
```
cargo build --release
```

# Run the program
Navigate to the UltrasoniPiPiano folder
```
./run.sh
```

# Stopping the program from running

To stop the program from running in the background, run the following command:

```
sudo killall -9 ultrasonic_piano
```
