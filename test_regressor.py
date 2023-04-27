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

# Train a random forest regressor on the data
print("Loading the regressor parameters...")
regressor = joblib.load("./model.joblib")

# Predict the hg/ha_yield for a given input
# Area, Crop, Pesticides Used, Avg Temperature, Avg Rainfall
new_data = [['India', 'Wheat', 121.0, 16.37, 1485.0]]
new_data = ct.transform(new_data)
predicted_yield = regressor.predict(new_data)[0]

print(f"Estimated yield: {predicted_yield:.2f} tonnes")

# Data visualization
import matplotlib.pyplot as plt

# Predictions for various pesticide use
pesticide_data = data[['pesticides_tonnes', 'hg/ha_yield']].copy()
pesticide_data['predicted_yield'] = regressor.predict(ct.transform(data[['Area', 'Item', 'pesticides_tonnes', 'avg_temp', 'average_rain_fall_mm_per_year']]))

# Group by pesticide use and plot predicted vs actual yield
pesticide_group = pesticide_data.groupby('pesticides_tonnes')
for name, group in pesticide_group:
    plt.plot(group['hg/ha_yield'], group['predicted_yield'], marker='o', linestyle='', label=name)
plt.plot([0, 500], [0, 500], 'k--', label='Perfect prediction')
plt.legend(title='Pesticides Used (in tonnes)')
plt.xlabel('Actual yield (hg/ha)')
plt.ylabel('Predicted yield (hg/ha)')
plt.title('Random Forest Regression: Actual vs Predicted yield')
plt.show()

# Predictions for various average temperature ranges
temp_data = data[['avg_temp', 'hg/ha_yield']].copy()
temp_data['predicted_yield'] = regressor.predict(ct.transform(data[['Area', 'Item', 'pesticides_tonnes', 'avg_temp', 'average_rain_fall_mm_per_year']]))
temp_data['temp_range'] = pd.cut(data['avg_temp'], bins=range(10, 40, 5))

# Group by average temperature range and plot predicted vs actual yield
temp_group = temp_data.groupby('temp_range')
for name, group in temp_group:
    plt.plot(group['hg/ha_yield'], group['predicted_yield'], marker='o', linestyle='', label=name)
plt.plot([0, 500], [0, 500], 'k--', label='Perfect prediction')
plt.legend(title='Temperature Range (in deg C)')
plt.xlabel('Actual yield (hg/ha)')
plt.ylabel('Predicted yield (hg/ha)')
plt.title('Random Forest Regression: Actual vs Predicted yield')
plt.show()
