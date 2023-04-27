import pandas as pd
import numpy as np
from sklearn.compose import ColumnTransformer
from sklearn.preprocessing import OneHotEncoder
from sklearn.ensemble import RandomForestRegressor
import joblib
import seaborn as sns

# Load the CSV file into a Pandas DataFrame
data = pd.read_csv('yield_df.csv')

# Split the data into input features (X) and output variable (y)
X = data[['Area', 'Item', 'pesticides_tonnes', 'avg_temp', 'average_rain_fall_mm_per_year']]
y = data['hg/ha_yield']

# Create a ColumnTransformer to one-hot encode the categorical variables
ct = ColumnTransformer(transformers=[('encoder', OneHotEncoder(), [0, 1])])
X = ct.fit_transform(X)

print("Training the model...")
# Train a random forest regressor on the data
regressor = RandomForestRegressor(n_estimators=100, random_state=42)
regressor.fit(X, y)
print("Dumping the regressor parameters..")
joblib.dump(regressor, "model.joblib")
