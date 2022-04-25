import math
import json
from glom import glom
import pandas as pd
import nltk
from sklearn.feature_extraction.text import TfidfVectorizer
from nltk.corpus import stopwords
from sklearn.metrics.pairwise import linear_kernel
import pymongo

# Replace the uri string with your MongoDB deployment's connection string.
conn_str = "mongodb+srv://sohamwakade7:Soham%4027@cluster0.wnbre.mongodb.net/Kilowog"

client = pymongo.MongoClient(conn_str, tls=True,tlsAllowInvalidCertificates=True)
# print(client)
# dbname = client.list_database_names()
# print(dbname)
db=client['Kilowog']
collection=db['Users']

# def getuser(uid : str):
#     db = client['Kilowog']
#     collection = db['Users']
#     return(collection.find_one({'userid': uid }))

# print(getuser('1001'))


userdata = collection.find_one({'userid': '1001' }, {'_id': 0, 'clicks' : 1 })
clicks = pd.DataFrame(userdata)
# print(clicks)
impclick =clicks.loc[0, :]
# print((impclick))

userdf = pd.read_json('../outputs_kwe/iphone.json')

# with open('../outputs_kwe/lenovoIdeapad.json','r') as f:
#     data = json.loads(f.read())
userdf1 =pd.DataFrame(userdf['listings'].apply(lambda row: glom(row, 'name')))
print (userdf1)
id=[]
for i in range(len(userdf1)):
    id.append(i)
# print (id)
userdf1['id']=id
print(userdf1)

# train data
tf = TfidfVectorizer(analyzer='word', ngram_range=(1, 3), min_df=0, stop_words='english')
tfidf_matrix = tf.fit_transform(userdf1['listings'])


cosine_similarities = linear_kernel(tfidf_matrix, tfidf_matrix)

results = {}
cosine_similarities

for idx, row in userdf1.iterrows():
    similar_indices = cosine_similarities[idx].argsort()[:-10:-1]
    similar_items = [(cosine_similarities[idx][i], userdf1['id'][i]) for i in similar_indices]


    results[row['id']] = similar_items[1:]

print('done')
# Predict recommendation
def item(id):
    return userdf1.loc[userdf1['id'] == id]['listings'].tolist()[0].split(' - ')[0]


def recommend(item_id, num):
    print("Recommending " + str(num) + " products similar to " + item(item_id) + "...")
    print("-------")
    recs = results[item_id][:num]
    print (recs)
    for rec in recs:
        print("Recommended: " + item(rec[1]) + " (score:" + str(rec[0]) + ")")


recommend(item_id=1, num=5)
