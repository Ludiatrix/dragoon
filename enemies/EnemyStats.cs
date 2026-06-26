using Godot;

namespace ExampleProject
{
	[GlobalClass]
	public partial class EnemyStats : Resource
	{
		[Export]
		public int MaxHealth { get; set; }

		[Export]
		public Resource Model { get; set; }

		[Export]
		public Resource Ai { get; set; }

		public EnemyStats() : this(0, null, null) {}

		public EnemyStats(int maxHealth, Resource model, Resource ai)
		{
			MaxHealth = maxHealth;
			Model = model;
			Ai = ai;
		}
	}
}
