from operator import itemgetter
import numpy as np
import pandas as pd
from sklearn.ensemble import RandomForestRegressor, BaggingRegressor
from nltk.stem.snowball import SnowballStemmer

#RELEVANCE
def rvs(pjson):
    # print(pjson)
    # print(type(pjson))
    listingNames = list( map(lambda listing: listing['name'], pjson['listings'] ))

    stemmer = SnowballStemmer('english')

    df_train = pd.read_csv('data/train/samsungm31trainingdata.csv')

    df_test = pd.DataFrame({'id':list(range(1,len(listingNames) + 1)),'query':pjson['query'],'name':listingNames})
    #df_test = pd.read_csv(prodlist_csv)

    num_train = df_train.shape[0]

    def str_stemmer(s):
        return " ".join([stemmer.stem(word) for word in str(s).lower().split()])

    def str_common_word(str1, str2):
        return sum(int(str2.find(word)>=0) for word in str1.split())

    df_all = pd.concat((df_train, df_test), axis=0, ignore_index=True)

    df_all['query'] = df_all['query'].map(lambda x:str_stemmer(x))

    df_all['name'] = df_all['name'].map(lambda x:str_stemmer(x))

    df_all['len_of_query'] = df_all['query'].map(lambda x:len(x.split())).astype(np.int64)

    df_all['product_info'] = df_all['query']+"\t"+df_all['name']

    df_all['word_in_title'] = df_all['product_info'].map(lambda x:str_common_word(x.split('\t')[0],x.split('\t')[1]))

    df_all = df_all.drop(['query','name','product_info'],axis=1)

    df_train = df_all.iloc[:num_train]
    df_test = df_all.iloc[num_train:]

    id_test = df_test['id']

    y_train = df_train['relevance'].values #isme training data ka relevance values

    X_train = df_train.drop(['id','relevance'],axis=1).values #word in title and length of query of training data

    X_test = df_test.drop(['id','relevance'],axis=1).values #word in title and length of query of testing data

    #rf = RandomForestRegressor(n_estimators=15, max_depth=6, random_state=0)
    #clf = BaggingRegressor(rf, n_estimators=45, max_samples=0.1, random_state=25)

    rf = RandomForestRegressor(n_estimators=100,max_depth=6,random_state=0)
    clf = BaggingRegressor(rf, n_estimators=50, max_samples=0.5, random_state=25)
    clf.fit(X_train, y_train)
    y_pred = clf.predict(X_test)

    for i in range(0, len(y_pred)):
        # print(prodlist_json['listings'][i]['name'])
        # print(prodlist_json['listings'][i])
        pjson['listings'][i]['order'] = y_pred[i]

    listings = pjson['listings']
    listings.sort(key=itemgetter('order'), reverse=True)
    pjson['listings'] = listings

    # with open('data/out.json', "w") as outfile:
    #     json.dump(pjson, outfile,indent=4)

    return pjson

# relevanceSort(load_local_json(filePath))
