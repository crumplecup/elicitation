//! Financial statement generation.
//!
//! This module provides GAAP-compliant financial statement generation including:
//! - Income Statement (Revenue - Expenses = Net Income)
//! - Statement period handling (monthly, quarterly, annual)
//! - Comparative statements (current vs. prior period)
//! - Financial ratios and metrics

use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::ledger2::{AccountNumber, Amount};

// ─────────────────────────────────────────────────────────────
//  Statement Period
// ─────────────────────────────────────────────────────────────

/// Accounting period for financial statements.
///
/// Defines the time range for financial reporting. Common periods are
/// monthly, quarterly, and annual.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatementPeriod {
    /// Start date of the period (inclusive).
    start_date: NaiveDate,

    /// End date of the period (inclusive).
    end_date: NaiveDate,

    /// Description of the period (e.g., "Q1 2024", "January 2024", "FY 2024").
    description: String,
}

impl StatementPeriod {
    /// Creates a new statement period.
    pub fn new(start_date: NaiveDate, end_date: NaiveDate, description: impl Into<String>) -> Self {
        Self {
            start_date,
            end_date,
            description: description.into(),
        }
    }

    /// Creates a monthly period.
    pub fn monthly(year: i32, month: u32) -> Self {
        let start_date = NaiveDate::from_ymd_opt(year, month, 1).expect("Valid date");
        let end_date = if month == 12 {
            NaiveDate::from_ymd_opt(year, 12, 31).expect("Valid date")
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
                .expect("Valid date")
                .pred_opt()
                .expect("Valid date")
        };

        let month_name = match month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => panic!("Invalid month"),
        };

        Self::new(start_date, end_date, format!("{} {}", month_name, year))
    }

    /// Creates a quarterly period.
    pub fn quarterly(year: i32, quarter: u32) -> Self {
        let (start_month, end_month) = match quarter {
            1 => (1, 3),
            2 => (4, 6),
            3 => (7, 9),
            4 => (10, 12),
            _ => panic!("Invalid quarter (must be 1-4)"),
        };

        let start_date = NaiveDate::from_ymd_opt(year, start_month, 1).expect("Valid date");
        let end_date = if end_month == 12 {
            NaiveDate::from_ymd_opt(year, 12, 31).expect("Valid date")
        } else {
            NaiveDate::from_ymd_opt(year, end_month + 1, 1)
                .expect("Valid date")
                .pred_opt()
                .expect("Valid date")
        };

        Self::new(start_date, end_date, format!("Q{} {}", quarter, year))
    }

    /// Creates an annual (fiscal year) period.
    pub fn annual(year: i32) -> Self {
        let start_date = NaiveDate::from_ymd_opt(year, 1, 1).expect("Valid date");
        let end_date = NaiveDate::from_ymd_opt(year, 12, 31).expect("Valid date");

        Self::new(start_date, end_date, format!("FY {}", year))
    }

    /// Returns the start date.
    pub fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    /// Returns the end date.
    pub fn end_date(&self) -> NaiveDate {
        self.end_date
    }

    /// Returns the description.
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl std::fmt::Display for StatementPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

// ─────────────────────────────────────────────────────────────
//  Income Statement
// ─────────────────────────────────────────────────────────────

/// Income Statement - summary of revenues and expenses for a period.
///
/// The income statement shows:
/// ```text
/// Revenue
/// - Expenses
/// = Net Income
/// ```
///
/// # GAAP Compliance
///
/// - **Accrual basis**: Revenues and expenses recorded when earned/incurred
/// - **Matching principle**: Expenses matched to revenues in same period
/// - **Period assumption**: Results reported for specific time period
///
/// # Example
///
/// ```rust,ignore
/// use elicit_server::ledger2::{IncomeStatement, StatementPeriod};
///
/// let period = StatementPeriod::monthly(2024, 1);
/// let statement = ledger.income_statement(&period);
///
/// println!("Revenue: {}", statement.total_revenue());
/// println!("Expenses: {}", statement.total_expenses());
/// println!("Net Income: {}", statement.net_income());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeStatement {
    /// Period this statement covers.
    period: StatementPeriod,

    /// Revenue accounts and amounts.
    revenue: HashMap<AccountNumber, Amount>,

    /// Expense accounts and amounts.
    expenses: HashMap<AccountNumber, Amount>,

    /// Total revenue.
    total_revenue: Amount,

    /// Total expenses.
    total_expenses: Amount,

    /// Net income (revenue - expenses).
    net_income: Amount,
}

impl IncomeStatement {
    /// Creates a new income statement for a period.
    pub fn new(period: StatementPeriod) -> Self {
        Self {
            period,
            revenue: HashMap::new(),
            expenses: HashMap::new(),
            total_revenue: Amount::from_cents(0),
            total_expenses: Amount::from_cents(0),
            net_income: Amount::from_cents(0),
        }
    }

    /// Returns the statement period.
    pub fn period(&self) -> &StatementPeriod {
        &self.period
    }

    /// Returns revenue accounts.
    pub fn revenue(&self) -> &HashMap<AccountNumber, Amount> {
        &self.revenue
    }

    /// Returns expense accounts.
    pub fn expenses(&self) -> &HashMap<AccountNumber, Amount> {
        &self.expenses
    }

    /// Returns total revenue.
    pub fn total_revenue(&self) -> Amount {
        self.total_revenue
    }

    /// Returns total expenses.
    pub fn total_expenses(&self) -> Amount {
        self.total_expenses
    }

    /// Returns net income.
    pub fn net_income(&self) -> Amount {
        self.net_income
    }

    /// Adds revenue for an account.
    pub fn add_revenue(&mut self, account: AccountNumber, amount: Amount) {
        self.total_revenue = self.total_revenue + amount;
        self.revenue.insert(account, amount);
        self.recompute_net_income();
    }

    /// Adds expense for an account.
    pub fn add_expense(&mut self, account: AccountNumber, amount: Amount) {
        self.total_expenses = self.total_expenses + amount;
        self.expenses.insert(account, amount);
        self.recompute_net_income();
    }

    /// Recomputes net income from revenue and expenses.
    fn recompute_net_income(&mut self) {
        self.net_income = self.total_revenue - self.total_expenses;
    }

    /// Returns the gross profit margin (net income / revenue).
    ///
    /// Returns None if revenue is zero.
    pub fn profit_margin(&self) -> Option<f64> {
        if self.total_revenue.is_zero() {
            None
        } else {
            Some(self.net_income.cents() as f64 / self.total_revenue.cents() as f64)
        }
    }
}

impl std::fmt::Display for IncomeStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Income Statement - {}", self.period)?;
        writeln!(f, "─────────────────────────────")?;
        writeln!(f, "Revenue:      {}", self.total_revenue)?;
        writeln!(f, "Expenses:     {}", self.total_expenses)?;
        writeln!(f, "─────────────────────────────")?;
        writeln!(f, "Net Income:   {}", self.net_income)?;

        if let Some(margin) = self.profit_margin() {
            writeln!(f, "Profit Margin: {:.1}%", margin * 100.0)?;
        }

        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────
//  Comparative Statement
// ─────────────────────────────────────────────────────────────

/// Comparative income statement - compares current and prior periods.
///
/// Shows side-by-side comparison of revenue, expenses, and net income
/// for current period vs. prior period, with variance analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparativeIncomeStatement {
    /// Current period statement.
    current: IncomeStatement,

    /// Prior period statement.
    prior: IncomeStatement,

    /// Revenue variance (current - prior).
    revenue_variance: Amount,

    /// Expense variance (current - prior).
    expense_variance: Amount,

    /// Net income variance (current - prior).
    net_income_variance: Amount,
}

impl ComparativeIncomeStatement {
    /// Creates a comparative statement.
    pub fn new(current: IncomeStatement, prior: IncomeStatement) -> Self {
        let revenue_variance = current.total_revenue() - prior.total_revenue();
        let expense_variance = current.total_expenses() - prior.total_expenses();
        let net_income_variance = current.net_income() - prior.net_income();

        Self {
            current,
            prior,
            revenue_variance,
            expense_variance,
            net_income_variance,
        }
    }

    /// Returns the current period statement.
    pub fn current(&self) -> &IncomeStatement {
        &self.current
    }

    /// Returns the prior period statement.
    pub fn prior(&self) -> &IncomeStatement {
        &self.prior
    }

    /// Returns revenue variance.
    pub fn revenue_variance(&self) -> Amount {
        self.revenue_variance
    }

    /// Returns expense variance.
    pub fn expense_variance(&self) -> Amount {
        self.expense_variance
    }

    /// Returns net income variance.
    pub fn net_income_variance(&self) -> Amount {
        self.net_income_variance
    }

    /// Returns revenue growth percentage.
    ///
    /// Returns None if prior revenue is zero.
    pub fn revenue_growth(&self) -> Option<f64> {
        if self.prior.total_revenue().is_zero() {
            None
        } else {
            Some(self.revenue_variance.cents() as f64 / self.prior.total_revenue().cents() as f64)
        }
    }
}

impl std::fmt::Display for ComparativeIncomeStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Comparative Income Statement")?;
        writeln!(f, "─────────────────────────────────────────────────")?;
        writeln!(
            f,
            "{:20} {:>15} {:>15} {:>15}",
            "", "Current", "Prior", "Variance"
        )?;
        writeln!(f, "─────────────────────────────────────────────────")?;
        writeln!(
            f,
            "{:20} {:>15} {:>15} {:>15}",
            "Revenue",
            self.current.total_revenue(),
            self.prior.total_revenue(),
            self.revenue_variance
        )?;
        writeln!(
            f,
            "{:20} {:>15} {:>15} {:>15}",
            "Expenses",
            self.current.total_expenses(),
            self.prior.total_expenses(),
            self.expense_variance
        )?;
        writeln!(f, "─────────────────────────────────────────────────")?;
        writeln!(
            f,
            "{:20} {:>15} {:>15} {:>15}",
            "Net Income",
            self.current.net_income(),
            self.prior.net_income(),
            self.net_income_variance
        )?;

        if let Some(growth) = self.revenue_growth() {
            writeln!(f, "\nRevenue Growth: {:.1}%", growth * 100.0)?;
        }

        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────
//  Financial Ratios
// ─────────────────────────────────────────────────────────────

/// Financial ratios computed from balance sheet and income statement.
///
/// Common ratios for financial analysis:
/// - **Profitability**: Profit margin, ROA, ROE
/// - **Liquidity**: Current ratio, quick ratio
/// - **Leverage**: Debt-to-equity ratio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialRatios {
    /// Profit margin (net income / revenue).
    profit_margin: Option<f64>,

    /// Current ratio (current assets / current liabilities).
    current_ratio: Option<f64>,

    /// Debt-to-equity ratio (total liabilities / total equity).
    debt_to_equity: Option<f64>,
}

impl FinancialRatios {
    /// Creates financial ratios from amounts.
    pub fn new(
        net_income: Amount,
        revenue: Amount,
        current_assets: Amount,
        current_liabilities: Amount,
        total_liabilities: Amount,
        total_equity: Amount,
    ) -> Self {
        let profit_margin = if revenue.is_zero() {
            None
        } else {
            Some(net_income.cents() as f64 / revenue.cents() as f64)
        };

        let current_ratio = if current_liabilities.is_zero() {
            None
        } else {
            Some(current_assets.cents() as f64 / current_liabilities.cents() as f64)
        };

        let debt_to_equity = if total_equity.is_zero() {
            None
        } else {
            Some(total_liabilities.cents() as f64 / total_equity.cents() as f64)
        };

        Self {
            profit_margin,
            current_ratio,
            debt_to_equity,
        }
    }

    /// Returns the profit margin.
    pub fn profit_margin(&self) -> Option<f64> {
        self.profit_margin
    }

    /// Returns the current ratio.
    pub fn current_ratio(&self) -> Option<f64> {
        self.current_ratio
    }

    /// Returns the debt-to-equity ratio.
    pub fn debt_to_equity(&self) -> Option<f64> {
        self.debt_to_equity
    }
}

impl std::fmt::Display for FinancialRatios {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Financial Ratios")?;
        writeln!(f, "─────────────────────────────")?;

        if let Some(margin) = self.profit_margin {
            writeln!(f, "Profit Margin:     {:.1}%", margin * 100.0)?;
        } else {
            writeln!(f, "Profit Margin:     N/A")?;
        }

        if let Some(ratio) = self.current_ratio {
            writeln!(f, "Current Ratio:     {:.2}", ratio)?;
        } else {
            writeln!(f, "Current Ratio:     N/A")?;
        }

        if let Some(ratio) = self.debt_to_equity {
            writeln!(f, "Debt-to-Equity:    {:.2}", ratio)?;
        } else {
            writeln!(f, "Debt-to-Equity:    N/A")?;
        }

        Ok(())
    }
}
