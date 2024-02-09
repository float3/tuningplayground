import re
import sys

def convert_ts_fraction_table_to_rust_array(ts_content: str) -> list:
    pattern = re.compile(r'const\s+(\w+):\s*FractionTable\s*=\s*\{(.*?)\};', re.DOTALL)
    
    named_fraction_tables = pattern.findall(ts_content)

    rust_arrays = []
    
    for name, table in named_fraction_tables:

        fraction_pairs = process_strings(table)

        rust_array_entries = [f"Fraction({numerator}, {denominator})" for numerator, denominator in fraction_pairs]
        
        rust_array = f"const {name.upper()}: [Fraction; {len(rust_array_entries)}] = [\n    " + ",\n    ".join(rust_array_entries) + "\n];"
        rust_arrays.append(rust_array)
    
    return rust_arrays

def process_ts_file_and_update_rust_file(ts_file_path: str, rust_file_path: str):
    try:
        with open(ts_file_path, 'r') as ts_file:
            ts_content = ts_file.read()
        
        rust_arrays = convert_ts_fraction_table_to_rust_array(ts_content)
        
        with open(rust_file_path, 'a') as rust_file:
            for rust_array in rust_arrays:
                rust_file.write(rust_array + "\n\n")
        print(f"Successfully updated {rust_file_path} with TypeScript fraction tables from {ts_file_path}.")
    except Exception as e:
        print(f"An error occurred: {e}")

def process_strings(input_string):
    lines = input_string.split('\n')
    
    result = []
    
    for line in lines:
        parts = line.split(':')
        if len(parts) > 1:
            sub_parts = parts[1].split('/')
            if len(sub_parts) > 1:
                result.append((sub_parts[0].strip(), sub_parts[1].strip()))
    
    return result


if __name__ == "__main__":
    ts_file_path = "playground.ts" 
    rust_file_path = "src/tables.rs"

    process_ts_file_and_update_rust_file(ts_file_path, rust_file_path)
