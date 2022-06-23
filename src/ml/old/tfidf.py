import collections.abc
import math
import json
from glom import glom
import numpy
import pandas as pd
import nltk
from sklearn.feature_extraction.text import TfidfVectorizer
from nltk.corpus import stopwords
from sklearn.metrics.pairwise import linear_kernel

userdf = pd.read_json('lenovoideapad.json')

with open('lenovoIdeapad.json','rb') as f:
    data = json.loads(f.read())
userdf1 =pd.DataFrame(userdf['listings'].apply(lambda row: glom(row, 'name')))
# print (userdf1)

userdf2 = pd.read_json('iphone.json')

with open('iphone.json','rb') as f:
    data = json.loads(f.read())
userdf3 =pd.DataFrame(userdf2['listings'].apply(lambda row: glom(row, 'name')))
# print (userdf3)

userdf4 = pd.read_json('geyser.json')

with open('geyser.json','rb') as f:
    data = json.loads(f.read())
userdf5 =pd.DataFrame(userdf4['listings'].apply(lambda row: glom(row, 'name')))
# print (userdf5)


combdf=[userdf1,userdf3,userdf5]
finaldf=pd.concat(combdf, ignore_index=True)
id=[]
for i in range(len(finaldf)):
    id.append(i)
# print (id)
finaldf['id']=id
print(finaldf)

# train data
tf = TfidfVectorizer(analyzer='word', ngram_range=(1, 3), min_df=0, stop_words='english')
tfidf_matrix = tf.fit_transform(finaldf['listings'])

cosine_similarities = linear_kernel(tfidf_matrix, tfidf_matrix)

results = {}
cosine_similarities

for idx, row in finaldf.iterrows():
    similar_indices = cosine_similarities[idx].argsort()[:-10:-1]
    similar_items = [(cosine_similarities[idx][i], finaldf['id'][i]) for i in similar_indices]
    # First item is the item itself, so remove it.
    # Each dictionary entry is like: [(1,2), (3,4)], with each tuple being (score, item_id)
    results[row['id']] = similar_items[1:]
print(results)


# Predict recommendation
def item(id):
    return finaldf.loc[finaldf['id'] == id]['listings'].tolist()[0].split(' - ')[0]


def recommend(item_id, num):
    print("Recommending " + str(num) + " products similar to " + item(item_id) + "...")
    print("-------")
    recs = results[item_id][:num]
    print (recs)
    for rec in recs:
        print("Recommended: " + item(rec[1]) + " (score:" + str(rec[0]) + ")")


recommend(item_id=1, num=5)
