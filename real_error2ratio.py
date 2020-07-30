#!/usr/bin/env python

import csv
import argparse

import altair
import pandas

def mytheme():
    return {'usermeta': {'embedOptions': {'theme': 'dark'}}, 'config': {'view': {'continuousWidth': 1600, 'continuousHeight': 1200}}}

altair.themes.register('mydark', mytheme)
altair.themes.enable("mydark")

def main(args):

    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--id2ratio", required=True)
    parser.add_argument("-f", "--fasta")
    parser.add_argument("-I", "--id2error")
    parser.add_argument("-o", "--output", required=True)

    args = parser.parse_args(args)

    if "fasta" not in args or "id2error" not in args:
        print("fasta or id2error need to be set")
        return -1

    id2error = dict()
    if "fasta" in args:
        with open(args.fasta) as fh:
            for line in fh:
                if line.startswith('>'):
                    error = float(line.split("=")[-1][:-2])
                    id = line.split(" ")[0][1:]
                    id2error[id] = error
    elif "id2error" in args:
        with open(args.id2error) as fh:
            reader = csv.reader(fh)
            for row in reader:
                id2error[row[0]] = float(row[1])
            
    id2ratio = dict()
    with open(args.id2ratio) as fh:
        reader = csv.reader(fh)
        for row in reader:
            id2ratio[row[0]] = float(row[1])

    data = list()
    for key in id2error.keys():
        if key in id2ratio:
            data.append((key, id2error[key], id2ratio[key]))

    df = pandas.DataFrame(data, columns=['id', 'error', 'ratio'])

    fig = altair.Chart(df).mark_circle(size=10, clip=True).encode(
        x=altair.X('error', scale=altair.Scale(domain=(65, 100))),
        y='ratio'
    ).interactive()

    fig.save(args.output)
    
    return 0

if __name__ == '__main__':
    import sys

    exit(main(sys.argv[1:]))
