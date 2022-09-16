use std::vec::Vec;

/// [DO NOT CHANGE THIS STRUCT - YOU MAY USE IT AS YOU SEE FIT]
/// The normalized data is stored in the 2D Matrix (Vector of Vectors) as follow:
///
///         col1    col2    col3 ...
///    row1    x       x       x
///    row2    x       x       x
///    row3    x       x       x
///    ...
///
/// Each column describes a different feature of some dataset. Each row is an
/// entry in the dataset. There can be as many rows and as many columns, so do
/// not hardcode these indices.
///
/// The labels vector contains labels that correspond to the data on each row.
/// This means labels[i] is associated with the data in row[i].
pub struct Dataset {
    pub matrix: Vec<Vec<f64>>,
    pub labels: Vec<String>,
}

impl Dataset {
    /// [IMPLEMENT THIS FUNCTION]
    /// Predict the label given some piece of data. Find the closest point in the
    /// normalized data to the given input data. Return the label of the point
    /// closest to the given input.
    ///
    /// You can assume the given input data is standardized/normalized. You can
    /// also assume the given input contains the same number of features as your
    /// dataset.
    ///
    /// @param input - a vector of N pieces of data
    /// @return a String indicating the label of the piece of data closest in
    ///         distance to the given input
    pub fn predict(&self, input: &Vec<f64>) -> String {
        let mut min_dis: f64 = distance(input, &self.matrix[0]);
        let mut label: String = self.labels[0].clone();
        for i in 1..self.matrix.len() {
            let dis: f64 = distance(input, &self.matrix[i]);
            if dis < min_dis {
                min_dis = dis;
                label = self.labels[i].clone();
            }
        }
        label
    }
}

/// [DO NOT CHANGE THIS FUNCTION - YOU MAY USE IT AS YOU SEE FIT]
/// This function calculates the distance between two N-dimentional points.
///
/// Ex.:    a = (1, 1, 1, 1), b = (4, 4, 4, 4)
///     --> distance(a, b) = sqrt( 4 * ( 4 - 1 )^2 ) = 6
///
/// @param a - an N-dimentional vector of doubles
/// @param b - an N-dimentional vector of doubles
/// @return the cartesian distance between the given vectors
pub fn distance(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    a.iter()
        .zip(b.iter())
        .fold(0.0f64, |sum, (x, y)| sum + (x - y).powf(2.0))
        .sqrt()
}

/// [IMPLEMENT THIS FUNCTION]
/// Normalize each value in the dataset based on COLUMNS. Recall that normalizing
/// indicates that you subtract the mean from a value and divide by the standard
/// deviation to obtain the normalized data 'z' in standard coordinates.
///
///                 x - mean
///            z =  --------
///                   std
///
/// You will modify/normalize the inputted data directly by applying the formula above
/// on each value in each column.
///
/// @param data - a 2D Matrix that will be normalized by the COLUMN
/// @param means - a Vector containing the mean of each COLUMN
/// @param stds - a Vector containing the standard deviation of each COLUMN
pub fn normalize(data: &mut Vec<Vec<f64>>, means: &Vec<f64>, stds: &Vec<f64>) {
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            data[i][j] = (data[i][j] - means[j]) / stds[j];
        }
    }
}

/// [IMPLEMENT THIS FUNCTION]
/// Find the mean of each COLUMN of the given 2D matrix (vector of vectors).
/// Refer to the comment on the 'Dataset' struct for the format of the provided
/// matrix.
///
/// @param data - a 2D Matrix of data
/// @return a vector that contains the mean of each COLUMN in the given dataset.
pub fn mean(data: &Vec<Vec<f64>>) -> Vec<f64> {
    println!("{:?}", data);
    let mut means: Vec<f64> = Vec::new();
    let col_len: usize = data[0].len();
    for j in 0..col_len {
        let mut col_sum: f64 = 0.0;
        for i in 0..data.len() {
            col_sum += data[i][j];
            
        }
        means.push(col_sum / (data.len() as f64));
        println!("means: {:?}", means);
    }
    means
}

/// [IMPLEMENT THIS FUNCTION]
/// Find the standard deviation of each COLUMN of the given 2D matrix (vector of
/// vectors). Refer to the comment on the 'Dataset' struct for the format of the
/// provided matrix. Recall standard deviation is found as follows:
///
///                    _____________________________________
///                   |   sum( ( x[i][j] - mean[j] )^2 )  
///         std =   \ |   -------------------------------
///                  \|                 N              
///
///         where i = the row of the data and
///               j = the column of the data
///
/// @param data - a 2D Matrix of data
/// @return a vector that contains the standard deviation of each COLUMN in the
///         given dataset.
pub fn std(data: &Vec<Vec<f64>>, means: &Vec<f64>) -> Vec<f64> {
    let mut stds: Vec<f64> = Vec::new();
    let col_len: usize = data[0].len();
    for j in 0..col_len {
        let mut dev_sum: f64 = 0.0;
        for i in 0..data.len() {
            dev_sum += (data[i][j] - means[j]).powf(2.0);
        }
        stds.push((dev_sum / data.len() as f64).sqrt());
        println!("stds: {:?}", stds);
    }
    stds
}
