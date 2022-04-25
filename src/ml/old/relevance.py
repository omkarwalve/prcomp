import numpy as np
import pandas as pd
from sklearn.ensemble import RandomForestRegressor, BaggingRegressor
from nltk.stem.snowball import SnowballStemmer

stemmer = SnowballStemmer('english')

df_train = pd.read_csv('data/samsungm31trainingdata.csv')
df_test = pd.read_csv('data/washMachine.csv')


num_train = df_train.shape[0]

def str_stemmer(s: str):
	return " ".join([stemmer.stem(word) for word in s.lower().split()])

def str_common_word(str1, str2):
	return sum(int(str2.find(word)>=0) for word in str1.split())


df_all = pd.concat((df_train, df_test), axis=0, ignore_index=True)


df_all['query'] = df_all['query'].map(lambda x:str_stemmer(x))


df_all['name'] = df_all['name'].map(lambda x:str_stemmer(x))



df_all['len_of_query'] = df_all['query'].map(lambda x:len(x.split())).astype(np.int64)

df_all['product_info'] = df_all['query']+"\t"+df_all['name']

df_all['word_in_title'] = df_all['product_info'].map(lambda x:str_common_word(x.split('\t')[0],x.split('\t')[1]))
#print(df_all.loc[[9]])

df_all = df_all.drop(['query','name','product_info'],axis=1)

df_train = df_all.iloc[:num_train]
df_test = df_all.iloc[num_train:]

id_test = df_test['id']

y_train = df_train['relevance'].values #isme training data ka relevance values

X_train = df_train.drop(['id','relevance'],axis=1).values #word in title and length of query columns(training data)
X_test = df_test.drop(['id','relevance'],axis=1).values #word in title and length of query columns of testing data

#rf = RandomForestRegressor(n_estimators=15, max_depth=6, random_state=0)
#clf = BaggingRegressor(rf, n_estimators=45, max_samples=0.1, random_state=25)

#rf = RandomForestRegressor(n_estimators=8,max_depth=6,random_state=0)
#clf = BaggingRegressor(rf, n_estimators=8, max_samples=0.9, random_state=25)

rf = RandomForestRegressor(n_estimators=50,max_depth=6,random_state=0)
clf = BaggingRegressor(rf, n_estimators=10, max_samples=0.5, random_state=25)
clf.fit(X_train, y_train)
y_pred = clf.predict(X_test)
#y_pred.sort(reverse=True)
print(y_pred.sort(reverse=True))
#pd.DataFrame({"id": id_test, "relevance": y_pred}).to_csv('data/washMachine.csv', index=False)
