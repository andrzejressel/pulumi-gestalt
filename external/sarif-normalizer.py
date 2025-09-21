#!/usr/bin/env python3
"""
SARIF Path Converter - Convert Windows paths to Linux paths in SARIF files
"""

import json
import re
import argparse
import sys
from pathlib import Path

def convert_windows_path_to_linux(path_string):
    """
    Convert Windows-style path to Linux-style path
    
    Examples:
    - C:\\Users\\user\\project\\file.py -> /c/Users/user/project/file.py
    - D:\\workspace\\src\\main.cpp -> /d/workspace/src/main.cpp
    - src\\components\\Button.tsx -> src/components/Button.tsx
    - utils\\helpers\\format.js -> utils/helpers/format.js
    - .\\relative\\path\\file.txt -> ./relative/path/file.txt
    - ..\\parent\\file.js -> ../parent/file.js
    """
    if not isinstance(path_string, str):
        return path_string
    
    # Convert backslashes to forward slashes
    converted_path = path_string.replace('\\', '/')
    
    # Handle absolute Windows paths (C:, D:, etc.)
    drive_pattern = r'^([A-Za-z]):'
    match = re.match(drive_pattern, converted_path)
    if match:
        drive_letter = match.group(1).lower()
        converted_path = f"/{drive_letter}{converted_path[2:]}"
    
    return converted_path

def process_sarif_object(obj):
    """
    Recursively process SARIF object and convert Windows paths to Linux paths
    """
    if isinstance(obj, dict):
        for key, value in obj.items():
            # Common SARIF fields that contain file paths
            if key in ['uri', 'uriBaseId', 'baseUri', 'file', 'path', 'target']:
                if isinstance(value, str):
                    obj[key] = convert_windows_path_to_linux(value)
            else:
                obj[key] = process_sarif_object(value)
    elif isinstance(obj, list):
        for i, item in enumerate(obj):
            obj[i] = process_sarif_object(item)
    
    return obj

def convert_sarif_file(input_file, output_file=None):
    """
    Convert Windows paths to Linux paths in a SARIF file
    """
    try:
        # Read the SARIF file
        with open(input_file, 'r', encoding='utf-8') as f:
            sarif_data = json.load(f)
        
        # Process the SARIF data
        converted_data = process_sarif_object(sarif_data)
        
        # Determine output file
        if output_file is None:
            input_path = Path(input_file)
            output_file = input_path.parent / f"{input_path.stem}_linux{input_path.suffix}"
        
        # Write the converted SARIF file
        with open(output_file, 'w', encoding='utf-8') as f:
            json.dump(converted_data, f, indent=2, ensure_ascii=False)
        
        print(f"Successfully converted {input_file} -> {output_file}")
        return True
        
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON in {input_file}: {e}")
        return False
    except FileNotFoundError:
        print(f"Error: File {input_file} not found")
        return False
    except Exception as e:
        print(f"Error processing {input_file}: {e}")
        return False

def main():
    parser = argparse.ArgumentParser(
        description="Convert Windows-style paths to Linux-style paths in SARIF files"
    )
    parser.add_argument("input_file", help="Input SARIF file path")
    parser.add_argument("-o", "--output", help="Output SARIF file path (optional)")
    parser.add_argument("--in-place", action="store_true", 
                       help="Modify the input file in place")
    
    args = parser.parse_args()
    
    if args.in_place:
        output_file = args.input_file
    else:
        output_file = args.output
    
    success = convert_sarif_file(args.input_file, output_file)
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()