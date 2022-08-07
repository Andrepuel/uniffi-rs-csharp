using System;
using System.Runtime.InteropServices;
using System.Threading.Tasks;

[StructLayout(LayoutKind.Sequential)]
struct _UniFFIRustBuffer {
    int capacity;
    int len;
    IntPtr data;
}

[StructLayout(LayoutKind.Sequential)]
struct _UniFFIRustCallStatus {
    sbyte code;
    _UniFFIRustBuffer error_buf;
}

class _UniFFILib {
    {%- for func in ci.iter_ffi_function_definitions() %}
        [DllImport("{{ config.cdylib_name() }}", CallingConvention = CallingConvention.Cdecl)]
        public extern static {{ func.return_type()|optional_ffi_type_name }} {{ func.name() }} (
            {%- for arg in func.arguments() %}
                {{ arg.type_().borrow()|ffi_type_name }} {{ arg.name() }},
            {%- endfor %}
            ref _UniFFIRustCallStatus callStatus
        );
    {%- endfor %}
}


class _UniFFIConverterU64  {
    static public ulong Lift(ulong a) {
        return a;
    }
}

class _UniFFIConverterBool {
    static public bool Lift(sbyte a) {
        return a > 0;
    }
}

public class {{ config.class_name() }} {
    {%- for func in ci.function_definitions() %}
        static public {{ func.return_type()|optional_type_name }} {{ func.name().to_upper_camel_case() }} (
            {%- for arg in func.arguments() %}
                {%- if loop.index0 > 0 %},{%- endif %}
                {{ arg.type_().borrow()|type_name }} {{ arg.name().to_lower_camel_case() }}
            {%- endfor %}
            ) {
                _UniFFIRustCallStatus _uniffiCallStatus = new _UniFFIRustCallStatus();
                return {{ func.return_type()|optional_lift_function }}(
                    _UniFFILib.{{ func.ffi_func().name() }}(
                        {%- for arg in func.arguments() %}
                            {{ arg.name().to_lower_camel_case() }},
                        {%- endfor %}
                        ref _uniffiCallStatus
                    )
                );
            }
    {%- endfor %}
}