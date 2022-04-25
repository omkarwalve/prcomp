import collections.abc
import math
import json
import numpy
import pandas as pd
import nltk
import re
from itertools import combinations
from sklearn.feature_extraction.text import TfidfVectorizer
from nltk.corpus import stopwords
from sklearn.metrics.pairwise import cosine_similarity
from relv import rvs as format_json

def genValidQueries(clicks):
    wordfreqs = {}
    queryStrings = []
    # clicks = [ 'Lenovo IdeaPad Slim 5 11th Gen Intel Core i5 15.6" FHD IPS Thin & Light Laptop(16GB/512GB SSD/Windows 11/Office 2021/Backlit/Fingerprint Reader/3months Xbox Game Pass/Graphite Grey/1.66Kg),82FGO1H9IN',
    #            'Lenovo IdeaPad Slim 3 10th Gen Intel Core i3 15.6 HD Thin and Light Laptop (8GB/1TB HDD/Windows 11/MS Office 2021/2Yr Warranty/Platinum Grey/1.7Kg), 81WB01E9IN',
    #            'Lenovo IdeaPad 3 Core i3 11th Gen - (8 GB/256 GB SSD/Windows 11 Home) 14ITL05 Thin and Light Laptop',
    #            'Lenovo IdeaPad Slim 5 11th G',
    #            'APPLE iPhone 13 (Green, 128 GB)',
    #            'APPLE iPhone SE (White, 128 GB)',
    #            'APPLE iPhone 12 (Black, 128 GB)',
    #            'APPLE iPhone 12 Mini (Black, 64 GB)',
    #            'APPLE iPhone SE (Red, 128 GB)']

    for item in clicks:
        for word in re.split("\W+", item):
            if word != '':
                wordCount = item.count(word)
                wordfreqs[word] = wordfreqs.get(word, 0) + 1

    maxWordFreqs = sorted(wordfreqs.items(), key=lambda occurrence: occurrence[1], reverse=True)[0:2]

    combinationList = map(lambda x: x[0], maxWordFreqs)
    # print(list(combinationList))
    for combination in combinations(combinationList, 2):
        [firstWord, secondWord] = combination

        for item in clicks:
            if firstWord in item and secondWord in item:
                queryStrings.append(f'{firstWord}+{secondWord}')
                break

    # print(maxWordFreqs)
    # print(queryStrings)
    return queryStrings

def rcSort(pjson, clk):
    # JSON => DataFrame
    listingNames = list( map( lambda listing: listing['name'], pjson['listings'] ) )
    pnames = pd.DataFrame({ 'id': list(range(1,len(listingNames) + 1)), 'pname': listingNames })

    tf = TfidfVectorizer(analyzer='word', ngram_range=(1, 3), min_df=0, stop_words='english')
    matrix = tf.fit_transform(pnames['pname'])

    cosine_similarities = cosine_similarity(matrix)

    results = {}

    for idx, row in pnames.iterrows():
        similar_indices = cosine_similarities[idx].argsort()[:-10:-1]
        similar_items = [(cosine_similarities[idx][i], pnames['id'][i]) for i in similar_indices]
        # First item is the item itself, so remove it.
        # Each dictionary entry is like: [(1,2), (3,4)], with each tuple being (score, item_id)
        results[row['id']] = similar_items[1:]

    # idx = ideals(pnames, clk)
    # itms = recommend(idx, 5, results)
    # for i in range(0, len(pjson)):
    #     pjson['listings'][0]['name']['order'] = 
    return format_json(pjson)

def ideals(df,clk):
    mainId = pd.Series();
    for item in range(len(clk)):
        if mainId.empty:
            mainId = df[ df['pname'] == clk[item] ]['id']
    return mainId

def dfItem(df,idx):
    return df.loc[df['id'] == id]['pname'].tolist()[0].split(' - ')[0]

def recommend(item_id, num, res):
    recs = res[item_id][:num]
    items = []
    for rec in recs:
        # print("Recommended: " + item(rec[1]) + " (score:" + str(rec[0]) + ")")
        items.append((dfItem(item_id), rec[0]))
    return items

