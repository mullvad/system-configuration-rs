#!/usr/bin/env python3

"""
This file is a helper for code-genning sections of `generate_bindings.sh`
"""

import re
from typing import List, TypedDict


# ---- type & func defs ----
def tosnk(s: str):
    nound_pat = re.compile(r"(?<=[a-z])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])")
    s = nound_pat.sub('_', s).lower()
    return s


class Item(TypedDict):
    pure_cc: str
    pure_sc: str

    header_name: str
    header_path_var: str
    binding_path_var: str
    binding_name: str


def header_path_line(i: Item) -> str:
    return i["header_path_var"] + "=\"${SC_HEADER_PATH}/" + i["header_name"] + "\""


def binding_path_line(i: Item) -> str:
    return i["binding_path_var"] + "=\"${SC_BINDING_PATH}/" + i["binding_name"] + "\""


def header_path_block(ix: List[Item]) -> str:
    return "\n".join([header_path_line(i) for i in ix])


def binding_path_block(ix: List[Item]) -> str:
    return "\n".join([binding_path_line(i) for i in ix])


# ---- data processing ----
pure_cc_str = """CaptiveNetwork
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
pure_cc = pure_cc_str.split('\n')
items = [Item(pure_cc=s, pure_sc="", header_name=s + ".h", header_path_var="",
              binding_path_var="", binding_name="") for s in pure_cc]

# generate pure sc + binding name -> container
items = [Item(i, pure_sc=i["pure_cc"].replace("SC", "")) for i in items]
items = [Item(i, pure_sc=tosnk(i["pure_sc"])) for i in items]
items = [Item(i, binding_name=i["pure_sc"] + ".rs") for i in items]

# genrate path vars
items = [Item(i, header_path_var=i["pure_sc"].upper() + "_HEADER_PATH",
              binding_path_var=i["pure_sc"].upper() + "_BINDING_PATH") for i in items]

# GEN blocks
print("### header_path_block".upper())
print(header_path_block(items), "\n")

print("### binding_path_block".upper())
print(binding_path_block(items))
