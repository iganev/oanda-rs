#!/usr/bin/env python3
"""Compact dump of OANDA v20v3 schema definitions and operations."""
import json, sys

SPEC = '/home/ivan/IdeaProjects/oanda-rs/schema/json/v20v3.json'
with open(SPEC) as f:
    spec = json.load(f)
defs = spec['components']['schemas']

def typ(s):
    if not isinstance(s, dict): return '?'
    if '$ref' in s: return s['$ref'].split('/')[-1]
    t = s.get('type', '')
    if t == 'array': return f"[{typ(s.get('items', {}))}]"
    if 'oneOf' in s: return 'oneOf(' + '|'.join(typ(x) for x in s['oneOf']) + ')'
    if t == 'object' and 'properties' in s: return 'INLINE_OBJ'
    e = s.get('enum')
    if e and len(e) == 1: return f'const:{e[0]}'
    fmt = s.get('format')
    return f"{t}({fmt})" if fmt else t

def dump_def(name):
    s = defs.get(name)
    if s is None:
        print(f'!! {name}: NOT FOUND'); return
    req = set(s.get('required', []))
    print(f'== {name} ({typ(s) if "properties" not in s else "object"})')
    if 'enum' in s:
        print('   enum:', ' '.join(map(str, s['enum'])))
    if 'oneOf' in s:
        print('   oneOf:', ' '.join(typ(x) for x in s['oneOf']))
        d = s.get('discriminator', {})
        if d: print('   discriminator:', d.get('propertyName'), json.dumps(d.get('mapping', {})) if d.get('mapping') else '')
    for p, ps in s.get('properties', {}).items():
        star = '*' if p in req else ' '
        desc = (ps.get('description') or '').split('. ')[0][:100] if isinstance(ps, dict) else ''
        enum = ''
        if isinstance(ps, dict) and 'enum' in ps and len(ps['enum']) > 1:
            enum = ' enum[' + ','.join(map(str, ps['enum'][:40])) + ']'
        null = ' NULLABLE' if isinstance(ps, dict) and ps.get('nullable') else ''
        print(f'  {star}{p}: {typ(ps)}{enum}{null}  -- {desc}')

def dump_op(path_sub, method=None):
    for path, ops in spec['paths'].items():
        if path_sub not in path: continue
        for m, op in ops.items():
            if m == 'parameters': continue
            if method and m != method: continue
            print(f'== {m.upper()} {path}  [{op.get("operationId")}]')
            print('   summary:', (op.get('summary') or '').strip()[:120])
            params = ops.get('parameters', []) + op.get('parameters', [])
            for prm in params:
                if '$ref' in prm:
                    prm = spec['components']['parameters'][prm['$ref'].split('/')[-1]]
                req = '*' if prm.get('required') else ' '
                sch = prm.get('schema', {})
                print(f'  {req}{prm["in"]}:{prm["name"]}: {typ(sch)}  -- {(prm.get("description") or "").split(". ")[0][:90]}')
            rb = op.get('requestBody')
            if rb:
                sch = rb.get('content', {}).get('application/json', {}).get('schema', {})
                print(f'   body: {typ(sch)}')
                if 'properties' in sch:
                    req = set(sch.get('required', []))
                    for p, ps in sch['properties'].items():
                        star = '*' if p in req else ' '
                        print(f'    {star}{p}: {typ(ps)}')
            for code, resp in op.get('responses', {}).items():
                if '$ref' in resp:
                    print(f'   resp {code}: -> {resp["$ref"].split("/")[-1]}'); continue
                sch = resp.get('content', {}).get('application/json', {}).get('schema', {})
                hdrs = ','.join(resp.get('headers', {}).keys())
                print(f'   resp {code}: {typ(sch)}' + (f'  headers[{hdrs}]' if hdrs else ''))
                if 'properties' in sch:
                    req = set(sch.get('required', []))
                    for p, ps in sch['properties'].items():
                        star = '*' if p in req else ' '
                        print(f'    {star}{p}: {typ(ps)}')

mode = sys.argv[1]
if mode == 'def':
    for n in sys.argv[2:]: dump_def(n)
elif mode == 'op':
    dump_op(sys.argv[2], sys.argv[3] if len(sys.argv) > 3 else None)
elif mode == 'list':
    print(' '.join(sorted(defs.keys())))
