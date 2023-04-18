# Baggage-Conveyor-Security-Handling
This is a baggage handling system for security to detect bags with suspicious materials passing on the conveyor belt.
The conveyor belts are rolling bags. Some of these bag contain suspicious materials.
On detection of these suspicious bags, the belt is stopped and the security checks the bag.
After checking the bag, security enters the stopped belt number on the system and the belt resumes rolling.

The source code is written in Rust language. This employs multithreading for processing bags on different belts.
Each belt is run using different thread. 
There is a separate thread to read stopped conveyor belt number to resume rolling.
