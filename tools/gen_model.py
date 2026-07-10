#!/usr/bin/env python3
"""Generates Rust model code from OANDA v20v3 schema definitions.

Usage: gen_model.py <DefName> [<DefName>...]
Prints struct / string_enum! code to stdout for review and hand-finishing.
"""
import json, re, sys, textwrap

SPEC = '/home/ivan/IdeaProjects/oanda-rs/schema/json/v20v3.json'
spec = json.load(open(SPEC))
DEFS = spec['components']['schemas']

NAME_MAP = {
    'AccountID': 'AccountId', 'TransactionID': 'TransactionId', 'OrderID': 'OrderId',
    'TradeID': 'TradeId', 'ClientID': 'ClientId', 'RequestID': 'RequestId',
    'ClientRequestID': 'RequestId', 'DateTime': 'DateTime', 'InstrumentName': 'InstrumentName',
    'Currency': 'Currency', 'DecimalNumber': 'DecimalNumber', 'AccountUnits': 'AccountUnits',
    'PriceValue': 'PriceValue', 'ClientTag': 'ClientTag', 'ClientComment': 'ClientComment',
    'OrderSpecifier': 'OrderSpecifier', 'TradeSpecifier': 'TradeSpecifier',
}

def pascal(v):
    return ''.join(p.capitalize() for p in re.findall(r'[A-Za-z]+|\d+', str(v).replace('_', ' ').replace('-', ' ')))

KEYWORDS = {'type', 'move', 'ref', 'self'}

def snake(name):
    s = re.sub(r'(?<=[a-z0-9])([A-Z])', r'_\1', name)
    s = re.sub(r'(?<=[A-Z])([A-Z][a-z])', r'_\1', s)
    s = s.lower().replace('_i_ds', '_ids')
    return 'r#' + s if s in KEYWORDS else s

def prim_type(s):
    """Maps an inline primitive schema to a Rust type using format/description heuristics."""
    t = s.get('type')
    fmt = (s.get('format') or '') + ' ' + (s.get('description') or '')
    if t == 'boolean': return 'bool'
    if t == 'integer': return 'i64'
    if t == 'number': return 'f64'
    if t == 'string':
        if 'RFC 3339 representation' in fmt: return 'DateTime'
        if 'depends on the Instrument' in fmt: return 'PriceValue'
        if 'delimited by ":" characters' in fmt: return 'CandleSpecification'
        if 'OANDA-assigned User ID' in fmt: return 'String'
        if '{siteID}-{divisionID}' in fmt: return 'AccountId'
        if 'OANDA-assigned TransactionID' in fmt: return 'TransactionId'
        if 'OANDA-assigned OrderID' in fmt: return 'OrderId'
        if 'OANDA-assigned TradeID' in fmt: return 'TradeId'
        if 'base currency and quote currency delimited' in fmt: return 'InstrumentName'
        if 'ISO 4217 currency' in fmt: return 'Currency'
        if 'string representation of a Price' in fmt or 'string representation of the Price' in fmt: return 'PriceValue'
        if "precision provided depends on the Account's home currency" in fmt: return 'AccountUnits'
        if 'decimal number encoded as a string' in fmt: return 'DecimalNumber'
        if "client-provided identifier" in fmt: return 'ClientId'
        if 'Request ID' in fmt: return 'RequestId'
        if 'prefixed by the "@" symbol' in fmt:
            return 'TradeSpecifier' if 'Trade' in fmt else 'OrderSpecifier'
        return 'String'
    return 'serde_json::Value  /* TODO */'

def rust_type(s):
    if '$ref' in s:
        n = s['$ref'].split('/')[-1]
        return NAME_MAP.get(n, n)
    if s.get('type') == 'array':
        return f"Vec<{rust_type(s.get('items', {}))}>"
    if 'oneOf' in s:
        return 'serde_json::Value  /* TODO oneOf: %s */' % '|'.join(rust_type(x) for x in s['oneOf'])
    if s.get('type') == 'object' and 'properties' in s:
        return 'serde_json::Value  /* TODO inline object */'
    if 'enum' in s and len(s['enum']) == 1:
        return f'CONST:{s["enum"][0]}'
    if 'enum' in s:
        return 'String  /* TODO inline enum: %s */' % ','.join(map(str, s['enum'][:10]))
    return prim_type(s)

def doc_lines(text, indent):
    if not text: return []
    text = re.sub(r'\s+', ' ', text).strip()
    # escape rustdoc link brackets
    text = text.replace('[', '\\[').replace(']', '\\]')
    out = []
    for line in textwrap.wrap(text, width=76 - len(indent)):
        out.append(f'{indent}/// {line}')
    return out

def gen_enum(name, s):
    lines = []
    lines.append('string_enum! {')
    lines += doc_lines(s.get('description') or name, '    ')
    lines.append(f'    pub enum {name} {{')
    for v in s['enum']:
        lines.append(f'        {pascal(v)} => "{v}",')
    lines.append('    }')
    lines.append('}')
    return '\n'.join(lines)

def gen_struct(name, s):
    rust_name = NAME_MAP.get(name, name)
    lines = []
    lines += doc_lines(s.get('description') or f'The {name} representation.', '')
    lines.append('#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]')
    lines.append('#[non_exhaustive]')
    lines.append(f'pub struct {rust_name} {{')
    for prop, ps in s.get('properties', {}).items():
        field = snake(prop)
        ty = rust_type(ps)
        if not ty.startswith('CONST:'):
            lines += doc_lines(ps.get('description') or f'The `{prop}` field.', '    ')
        if ty.startswith('CONST:'):
            lines.append(f'    // type is pinned to "{ty[6:]}" by the enum wrapper')
            lines.append('')
            continue
        if ty.startswith('Vec<'):
            lines.append(f'    #[serde(rename = "{prop}", default, skip_serializing_if = "Vec::is_empty")]')
            lines.append(f'    pub {field}: {ty},')
        else:
            lines.append(f'    #[serde(rename = "{prop}", skip_serializing_if = "Option::is_none")]')
            lines.append(f'    pub {field}: Option<{ty}>,')
        lines.append('')
    if lines[-1] == '': lines.pop()
    lines.append('}')
    return '\n'.join(lines)

for name in sys.argv[1:]:
    s = DEFS.get(name)
    if s is None:
        print(f'// !! {name} NOT FOUND'); continue
    if 'enum' in s and s.get('type') == 'string':
        print(gen_enum(name, s))
    elif 'properties' in s:
        print(gen_struct(name, s))
    else:
        print(f'// {name}: not an object/enum — handle manually: {json.dumps(s)[:200]}')
    print()
