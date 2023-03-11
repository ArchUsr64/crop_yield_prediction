import pandas as pd
from sklearn.linear_model import LinearRegression
import matplotlib.pyplot as plt
import numpy as np

# Load the CSV file into a Pandas DataFrame
data = pd.read_csv('yield_df.csv')

# Split the data into input features (X) and output variable (y)
X = data[['Area', 'Item', 'pesticides_tonnes', 'avg_temp']].values
y = data['hg/ha_yield'].values

# Create a dictionary to map categorical variables to numerical values
area_map = {area: i for i, area in enumerate(data['Area'].unique())}
item_map = {item: i for i, item in enumerate(data['Item'].unique())}
# Map the categorical variables to numerical values using the dictionary
X[:, 0] = [area_map[item] for item in X[:, 0]]
X[:, 1] = [item_map[item] for item in X[:, 1]]

print(item_map)
# Train a linear regression model on the data
regressor = LinearRegression()
regressor.fit(X, y)

# Predict the ha_yield for a given input
# Area ID, Crop ID, Pesticides Used, Avg Temperature
new_data = [[area_map['India'], 5, 121.0, 16.37]]
predicted_yield = regressor.predict(new_data)[0]

print(f"Estimated yield: {predicted_yield:.2f} tonnes")
# Generate some sample data for pesticide use
pesticide_levels = np.linspace(0, 200, 50)
pesticide_data = [[area_map['India'], 5, p, 16.37] for p in pesticide_levels]

# Use the trained regressor to predict yields for the sample data
predicted_yields = regressor.predict(pesticide_data)

# Create a scatter plot of the predicted yields for different pesticide levels
plt.scatter(pesticide_levels, predicted_yields)
plt.xlabel('Pesticide Used (tonnes)')
plt.ylabel('Estimated Yield (tonnes)')
plt.title('Yield Prediction for Different Pesticide Levels')
plt.show()
