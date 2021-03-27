
## dynamic lib

cc -o libhl.so -m64  -L/opt/libjpeg-turbo/lib64 -shared include/pcre/pcre_chartables.o include/pcre/pcre_compile.o include/pcre/pcre_dfa_exec.o include/pcre/pcre_exec.o include/pcre/pcre_fullinfo.o include/pcre/pcre_globals.o include/pcre/pcre_newline.o include/pcre/pcre_string_utils.o include/pcre/pcre_tables.o include/pcre/pcre_xclass.o include/pcre/pcre16_ord2utf16.o include/pcre/pcre16_valid_utf16.o include/pcre/pcre_ucd.o src/gc.o src/std/array.o src/std/buffer.o src/std/bytes.o src/std/cast.o src/std/date.o src/std/error.o src/std/debug.o src/std/file.o src/std/fun.o src/std/maps.o src/std/math.o src/std/obj.o src/std/random.o src/std/regexp.o src/std/socket.o src/std/string.o src/std/sys.o src/std/types.o src/std/ucs2.o src/std/thread.o src/std/process.o src/std/track.o -lpthread -lm




## hl build

cc -Wall -O3 -I src -msse2 -mfpmath=sse -std=c11 -I include -I include/pcre -I include/mikktspace -I include/minimp3 -D LIBHL_EXPORTS -m64 -fPIC -pthread -fno-omit-frame-pointer -o hl src/code.o src/jit.o src/main.o src/module.o src/debugger.o src/profile.o -L. -lhl -lm -Wl,-rpath,. -Wl,--export-dynamic -Wl,--no-undefined  -ldl


## static lib

ar crs libhlvm.a  src/code.o src/jit.o  src/module.o src/debugger.o src/profile.o 


## hl2 build
## main.c + libhlvm.a

#cc  -Wall -O3 -I src -msse2 -mfpmath=sse -std=c11 -I include -I include/pcre -I include/mikktspace -I include/minimp3 -D LIBHL_EXPORTS -m64 -fPIC -pthread -fno-omit-frame-pointer -o hl2  main.c  -L. -lhlvm -lhl -lm -Wl,-rpath,. -Wl,--export-dynamic -Wl,--no-undefined  -ldl

#cc  -w -O3 -I src -msse2 -mfpmath=sse -std=c11 -I include -I include/pcre -I include/mikktspace -I include/minimp3 -D LIBHL_EXPORTS -m64 -fPIC -pthread -fno-omit-frame-pointer -o hl_static  main.c  -L. -lhlvm -lhl -lm -Wl,-rpath,. -Wl,--export-dynamic -Wl,--no-undefined  -ldl


## func call

#   `obj_resolve_field' に対する定義されていない参照です
#cc  -w -O3 -I src -msse2 -mfpmath=sse -std=c11 -I include -I include/pcre -I include/mikktspace -I include/minimp3 -D LIBHL_EXPORTS -m64 -fPIC -pthread -fno-omit-frame-pointer -o hl2  main2.c  -L. -lhlvm -lhl -lm -Wl,-rpath,. -Wl,--export-dynamic -Wl,--no-undefined  -ldl

cc -w -O3 -I src -msse2 -mfpmath=sse -std=c11 -I include -I include/pcre -I include/mikktspace -I include/minimp3 -D LIBHL_EXPORTS -m64 -fPIC -pthread -fno-omit-frame-pointer -o hl3 src/code.o src/jit.o  src/module.o src/debugger.o src/profile.o main.c -L. -lhl -lm -Wl,-rpath,. -Wl,--export-dynamic -Wl,--no-undefined  -ldl
