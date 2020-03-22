target remote :3333

# print demangled symbols
set print asm-demangle on

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break UserHardFault
break rust_begin_unwind

monitor arm semihosting enable
load
break main
continue
layout src
step

#target remote :3333
#set print asm-demangle on
#set print pretty on
#monitor tpiu config internal itm.txt uart off 8000000
#monitor itm port 0 on
#load
#break DefaultHandler
#break HardFault
#break main
#continue
#layout src
#continue