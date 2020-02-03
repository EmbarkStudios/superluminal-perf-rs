/*
BSD LICENSE

Copyright (c) 2019-2020 Superluminal. All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:

  * Redistributions of source code must retain the above copyright
    notice, this list of conditions and the following disclaimer.
  * Redistributions in binary form must reproduce the above copyright
    notice, this list of conditions and the following disclaimer in
    the documentation and/or other materials provided with the
    distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
#pragma once

#include <inttypes.h>

// When PERFORMANCEAPI_ENABLED is defined to 0, all calls to the PerformanceAPI (either through macro or direct function calls) will be compiled out.
#ifndef PERFORMANCEAPI_ENABLED
	#ifdef _WIN32
		#define	PERFORMANCEAPI_ENABLED 1
	#else
		#define	PERFORMANCEAPI_ENABLED 0
	#endif
#endif

/**
* See PerformanceAPI.h for the high level documentation on how to use this API
*/

#ifdef __cplusplus
extern "C" {
#endif

#if PERFORMANCEAPI_ENABLED
	/**
	 * Helper struct that is used to prevent calls to EndEvent from being optimized to jmp instructions as part of tail call optimization.
	 * You don't ever need to do anything with this as user of the API.
	 */
	typedef struct
	{
		int64_t SuppressTailCall[3];
	} PerformanceAPI_SuppressTailCallOptimization;

	/**
	 * Helper function to create an uint32_t color from 3 RGB values. The R, G and B values must be in range [0, 255].
	 * The resulting color can be passed to the BeginEvent function.
	 */
	#define PERFORMANCEAPI_MAKE_COLOR(R, G, B) ((((uint32_t)(R)) << 24) | (((uint32_t)(G)) << 16) | (((uint32_t)(B)) << 24))

	/**
	 * Use this define if you don't care about the color of an event and just want to use the default
	 */
	#define PERFORMANCEAPI_DEFAULT_COLOR 0xFFFFFFFF

	/**
	 * Set the name of the current thread to the specified thread name. 
	 *
	 * @param inThreadName The thread name as an UTF8 encoded string.
	 */
	void PerformanceAPI_SetCurrentThreadName(const char* inThreadName);

	/**
	 * Begin an instrumentation event with the specified ID and runtime data
	 *
	 * @param inID    The ID of this scope as an UTF8 encoded string. The ID for a specific scope must be the same over the lifetime of the program (see docs at the top of this file)
	 * @param inData  [optional] The data for this scope as an UTF8 encoded string. The data can vary for each invocation of this scope and is intended to hold information that is only available at runtime. See docs at the top of this file.
	 *							 Set to null if not available.
	 * @param inColor [optional] The color for this scope. The color for a specific scope is coupled to the ID and must be the same over the lifetime of the program
	 *							 Set to PERFORMANCEAPI_DEFAULT_COLOR to use default coloring.
	 *
	 */
	void PerformanceAPI_BeginEvent(const char* inID, const char* inData, uint32_t inColor);

	/**
	 * Begin an instrumentation event with the specified ID and runtime data
	 *
	 * @param inID    The ID of this scope as an UTF16 encoded string. The ID for a specific scope must be the same over the lifetime of the program (see docs at the top of this file)
	 * @param inData  [optional] The data for this scope as an UTF16 encoded string. The data can vary for each invocation of this scope and is intended to hold information that is only available at runtime. See docs at the top of this file.
	 						     Set to null if not available.
	 * @param inColor [optional] The color for this scope. The color for a specific scope is coupled to the ID and must be the same over the lifetime of the program
	 *							 Set to PERFORMANCEAPI_DEFAULT_COLOR to use default coloring.
	 */
	void PerformanceAPI_BeginEvent_Wide(const wchar_t* inID, const wchar_t* inData, uint32_t inColor);

	/**
	 * End an instrumentation event. Must be matched with a call to BeginEvent within the same function
	 * Note: the return value can be ignored. It is only there to prevent calls to the function from being optimized to jmp instructions as part of tail call optimization.
	 */
	PerformanceAPI_SuppressTailCallOptimization PerformanceAPI_EndEvent();
#else
	#define PERFORMANCEAPI_MAKE_COLOR(R, G, B) 0xFFFFFFFF
	#define PERFORMANCEAPI_DEFAULT_COLOR 0xFFFFFFFF

	inline void PerformanceAPI_SetCurrentThreadName(const char* inThreadName) {}
	inline void PerformanceAPI_BeginEvent(const char* inID, const char* inData, uint32_t inColor) {}
	inline void PerformanceAPI_BeginEvent_Wide(const wchar_t* inID, const wchar_t* inData, uint32_t inColor) {}
	inline void PerformanceAPI_EndEvent() {}
#endif

#ifdef __cplusplus
} // extern "C"
#endif
