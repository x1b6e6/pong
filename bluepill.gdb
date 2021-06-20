target extended-remote | openocd -c 'gdb_port pipe' -f interface/stlink.cfg -f target/stm32f1x.cfg
monitor halt
monitor flash probe 0
monitor flash protect 0 0 15 off
load
monitor flash protect 0 0 15 on
monitor reset halt
