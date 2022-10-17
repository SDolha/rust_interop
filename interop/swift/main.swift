import Foundation
import RustInterop

func cString(_ value: String) -> UnsafeMutablePointer<Int8> {
    UnsafeMutablePointer<Int8>(mutating: NSString(string: value).utf8String!) 
}
func string(_ value: UnsafeMutablePointer<Int8>?) -> String {
    String(cString: value!)
}

let original = CObject(flag: 0, data: cString("Object Name=\"Swift🐦\""))
let updated = c_update(original)
print("flag:", updated.flag)
print(string(updated.data))
