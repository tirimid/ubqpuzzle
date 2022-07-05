# Install script for directory: /home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "0")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build/libSDL2.a")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE SHARED_LIBRARY FILES
    "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build/libSDL2-2.0.so.0.16.0"
    "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build/libSDL2-2.0.so.0"
    )
  foreach(file
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libSDL2-2.0.so.0.16.0"
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libSDL2-2.0.so.0"
      )
    if(EXISTS "${file}" AND
       NOT IS_SYMLINK "${file}")
      if(CMAKE_INSTALL_DO_STRIP)
        execute_process(COMMAND "/usr/bin/strip" "${file}")
      endif()
    endif()
  endforeach()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE SHARED_LIBRARY FILES "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build/libSDL2-2.0.so")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libSDL2-2.0.so" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libSDL2-2.0.so")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/usr/bin/strip" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libSDL2-2.0.so")
    endif()
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build/libSDL2main.a")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2Targets.cmake")
    file(DIFFERENT EXPORT_FILE_CHANGED FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2Targets.cmake"
         "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build/CMakeFiles/Export/lib/cmake/SDL2/SDL2Targets.cmake")
    if(EXPORT_FILE_CHANGED)
      file(GLOB OLD_CONFIG_FILES "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2Targets-*.cmake")
      if(OLD_CONFIG_FILES)
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2/SDL2Targets.cmake\" will be replaced.  Removing files [${OLD_CONFIG_FILES}].")
        file(REMOVE ${OLD_CONFIG_FILES})
      endif()
    endif()
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2" TYPE FILE FILES "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build/CMakeFiles/Export/lib/cmake/SDL2/SDL2Targets.cmake")
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2" TYPE FILE FILES "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build/CMakeFiles/Export/lib/cmake/SDL2/SDL2Targets-release.cmake")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xDevelx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/SDL2" TYPE FILE FILES
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/SDL2Config.cmake"
    "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build/SDL2ConfigVersion.cmake"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/SDL2" TYPE FILE FILES
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_assert.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_atomic.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_audio.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_bits.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_blendmode.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_clipboard.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_config_android.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_config_iphoneos.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_config_macosx.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_config_minimal.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_config_os2.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_config_pandora.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_config_psp.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_config_windows.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_config_winrt.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_config_wiz.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_copying.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_cpuinfo.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_egl.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_endian.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_error.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_events.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_filesystem.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_gamecontroller.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_gesture.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_haptic.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_hints.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_joystick.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_keyboard.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_keycode.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_loadso.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_locale.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_log.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_main.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_messagebox.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_metal.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_misc.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_mouse.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_mutex.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_name.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_opengl.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_opengl_glext.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_opengles.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_opengles2.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_opengles2_gl2.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_opengles2_gl2ext.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_opengles2_gl2platform.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_opengles2_khrplatform.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_pixels.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_platform.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_power.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_quit.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_rect.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_render.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_revision.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_rwops.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_scancode.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_sensor.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_shape.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_stdinc.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_surface.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_system.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_syswm.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_test.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_test_assert.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_test_common.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_test_compare.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_test_crc32.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_test_font.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_test_fuzzer.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_test_harness.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_test_images.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_test_log.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_test_md5.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_test_memory.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_test_random.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_thread.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_timer.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_touch.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_types.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_version.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_video.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/SDL_vulkan.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/begin_code.h"
    "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/include/close_code.h"
    "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build/include/SDL_config.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  
          execute_process(COMMAND /usr/bin/cmake -E create_symlink
            "libSDL2-2.0.so" "libSDL2.so"
            WORKING_DIRECTORY "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE FILE FILES "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build/libSDL2.so")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig" TYPE FILE FILES "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build/sdl2.pc")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/bin" TYPE PROGRAM FILES "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build/sdl2-config")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/share/aclocal/sdl2.m4")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/share/aclocal" TYPE FILE FILES "/home/dmitrii/.cargo/registry/src/github.com-1ecc6299db9ec823/fermium-20016.1.1/SDL2-2.0.16/sdl2.m4")
endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/home/dmitrii/Desktop/programming/ubqpuzzle/game/target/debug/build/fermium-c3b8d26315109329/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
