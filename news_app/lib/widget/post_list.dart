import 'package:flutter/material.dart';
import 'package:news_app/model/news.dart';
import 'package:news_app/widget/news_detail_page.dart';

class PostList extends StatelessWidget {
  const PostList({super.key, required this.posts});

  final List<News> posts;

  @override
  Widget build(BuildContext context) {
    if (posts.isEmpty) {
      return const Center(child: Text('No news available'));
    }

    return ListView.builder(
      itemCount: posts.length,
      itemBuilder: (context, index) {
        final post = posts[index];
        return GestureDetector(
          onTap: () {
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (context) => NewsDetailPage(news: post),
              ),
            );
          },
          child: Card(
            margin: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
            child: Padding(
              padding: const EdgeInsets.all(12.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    post.title,
                    style: Theme.of(context).textTheme.titleMedium,
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                  ),
                  const SizedBox(height: 8),
                  Text(
                    post.published,
                    maxLines: 3,
                    overflow: TextOverflow.ellipsis,
                    style: Theme.of(context).textTheme.bodyMedium,
                  ),
                ],
              ),
            ),
          ),
        );
      },
    );
  }
}
