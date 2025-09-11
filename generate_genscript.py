#!/usr/bin/env python3

"""
This file is a helper for code-genning sections of `generate_bindings.sh`
"""

import re
from typing import List, TypedDict


# ---- type & func defs ----
def to_snake_case(s: str):
    nound_pat = re.compile(r"(?<=[a-z])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])")
    s = nound_pat.sub('_', s).lower()
    return s


class Item(TypedDict):
    pure_camel_case: str
    pure_snake_case: str

    header_name: str
    header_path_var: str
    binding_path_var: str
    binding_name: str

    header_path_line: str
    binding_path_line: str


def grab(ix: List[Item], k: str) -> Item:
    return [i for i in ix if i["pure_camel_case"] == k][0]


def header_path_line(i: Item) -> str:
    return i["header_path_var"] + "=\"${SC_HEADER_PATH}/" + i["header_name"] + "\""


def binding_path_line(i: Item) -> str:
    return i["binding_path_var"] + "=\"${SC_BINDING_PATH}/" + i["binding_name"] + "\""


def header_path_block(ix: List[Item]) -> str:
    return "\n".join([i["header_path_line"] for i in ix])


def binding_path_block(ix: List[Item]) -> str:
    return "\n".join([i["binding_path_line"] for i in ix])


# ---- data processing ----
pure_camel_case_str = """CaptiveNetwork
DHCPClientPreferences
SCDynamicStore
SCDynamicStoreCopyDHCPInfo
SCDynamicStoreCopySpecific
SCDynamicStoreKey
SCNetwork
SCNetworkConfiguration
SCNetworkConnection
SCNetworkReachability
SCPreferences
SCPreferencesPath
SCPreferencesSetSpecific
SCSchemaDefinitions
SystemConfiguration"""

# pure cc + header name -> contaner
pure_camel_case = pure_camel_case_str.split('\n')
items = [Item(pure_camel_case=s, pure_snake_case="", header_name=s + ".h", header_path_var="",
              binding_path_var="", binding_name="", header_path_line="", binding_path_line="") for s in pure_camel_case]

# generate pure sc + binding name -> container
items = [Item(i, pure_snake_case=i["pure_camel_case"].replace("SC", ""))
         for i in items]
items = [Item(i, pure_snake_case=to_snake_case(i["pure_snake_case"]))
         for i in items]
items = [Item(i, binding_name=i["pure_snake_case"] + ".rs") for i in items]

# genrate path vars
items = [Item(i, header_path_var=i["pure_snake_case"].upper() + "_HEADER_PATH",
              binding_path_var=i["pure_snake_case"].upper() + "_BINDING_PATH") for i in items]

# generate path lines
items = [Item(i, header_path_line=header_path_line(
    i), binding_path_line=binding_path_line(i)) for i in items]

# ---- final output ----
print("### header_path_block".upper())
print(header_path_block(items), "\n")

print("### binding_path_block".upper())
print(binding_path_block(items), "\n")
